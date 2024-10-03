use crate::{limit_r, DataType, RLimited, Seekable, Writable};
use std::{
    cmp::min,
    io::{ErrorKind, Read, Result},
    vec,
};

/// # **This function is only for variable length signed integers, not normal integers!!!**
fn unsigned_to_signed(num: u128, length: u8, size: u8) -> i128 {
    let signed_max = (1 << (length * (8 * size - 1) - 1)) - 1;

    if num <= signed_max {
        println!(
            "pos size={} len={} unsigned=0x{:x} max=0x{:x}",
            size, length, num, signed_max
        );
        num as i128
    } else {
        let signed_min = -(signed_max as i128) - 1;
        println!(
            "neg size={} len={} unsigned=0x{:x} max=0x{:x} min=0x{:x}",
            size, length, num, signed_max, signed_min
        );

        signed_min + num as i128 + signed_min
    }
}

#[test]
fn test_unsigned_to_signed() {
    assert_eq!(unsigned_to_signed(0, 1, 1), 0);
    assert_eq!(unsigned_to_signed(0b0100_0000, 1, 1), -64);
    assert_eq!(unsigned_to_signed(0b0100_0001, 1, 1), -63);
    assert_eq!(unsigned_to_signed(0b0011_1111, 1, 1), 63);

    assert_eq!(unsigned_to_signed(0, 2, 1), 0);
    assert_eq!(unsigned_to_signed(0b0010_0000_0000_0000, 2, 1), -8192);
    assert_eq!(unsigned_to_signed(0b0010_0000_0000_0001, 2, 1), -8191);
    assert_eq!(unsigned_to_signed(0b0001_1111_1111_1111, 2, 1), 8191);
}

fn parse_vux(read_ux: &mut dyn FnMut(u8) -> Result<u128>, size: u8) -> Result<(u128, usize)> {
    let mut result = 0;
    let mut shift = 0u8;
    let mut i = 0;
    loop {
        i += 1;
        let bytes = read_ux(size)?;
        if bytes & (1 << (size * 8 - 1)) == 0 {
            result |= (bytes & !(1 << (size * 8 - 1))) << shift;
            break;
        }
        result |= (bytes & !(1 << (size * 8 - 1))) << shift;
        shift += size * 8 - 1;
    }
    Ok((result, i))
}

fn parse_vuxr(read_ux: &mut dyn FnMut(u8) -> Result<u128>, size: u8) -> Result<(u128, usize)> {
    let mut result = 0;
    let mut shift = 0u8;
    let mut bytes = [0u128; 16];
    let mut i = 0;
    loop {
        let b = read_ux(size)?;
        bytes[i] = b;
        i += 1;
        if b & (1 << (size * 8 - 1)) == 0 {
            break;
        }
    }
    bytes.reverse();
    let mut reached_data = false;
    for byte in bytes.iter() {
        if *byte == 0 && !reached_data {
            continue;
        }
        reached_data = true;
        result |= (*byte & !(1 << (size * 8 - 1))) << shift;
        shift += size * 8 - 1;
    }
    Ok((result, i))
}

/// Provides methods to read data from a source.
///
/// Although the trait can be implemented for any type that implements [`Read`] and [`Seekable`],
/// for most cases the [internal implementations](#implementors) are recommended.
pub trait Readable<'a>: Read + Seekable {
    /// Locks the source exclusively for the current process.
    /// This only has an effect on some sources, like files.
    ///
    /// ### Example
    ///
    /// ```should_panic
    /// use dh::recommended::*;
    ///
    /// let mut file1 = dh::file::open_r("tests/samples/000").unwrap();
    /// file1.lock(true).unwrap(); // this would block the thread until the file is unlocked
    ///
    /// let mut file2 = dh::file::open_r("tests/samples/000").unwrap();
    /// file2.lock(false).unwrap(); // fails, because the file is already locked
    /// ```
    fn lock(&mut self, block: bool) -> Result<()>;

    /// Unlocks the source for other processes.
    /// This happens automatically when the source goes out of scope, is closed or dropped.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut file = dh::file::open_r("tests/samples/000").unwrap();
    /// file.lock(true).unwrap();
    /// // do something with the file
    /// file.unlock().unwrap();
    /// ```
    fn unlock(&mut self) -> Result<()>;

    /// Closes the reader and can return the source if it was moved or references it.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![]);
    /// // do something with the reader
    /// reader.close().unwrap(); // if the reader goes out of scope, this happens automatically
    /// ```
    fn close(self) -> Result<Option<DataType<'a>>>;

    /// Limits the space the reader can read from.
    ///
    /// The reader will only be able to read from `pos` to `pos + length`.
    ///
    /// **Note:** The reader will automatically jump to the start of the limit here.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let mut limited = reader.limit(2, 4).unwrap();
    ///
    /// let size = limited.size().unwrap();
    /// assert_eq!(limited.read_bytes(size).unwrap(), vec![2, 3, 4, 5]);
    /// ```
    fn limit(&'a mut self, pos: u64, length: u64) -> Result<RLimited<'a, Self>> {
        limit_r(self, pos, length)
    }

    /// Copies data from the current position to a target at a specific position.
    ///
    /// This executes the [`copy`][Readable::copy] method at `pos` and then returns to the original position.
    fn copy_at(
        &mut self,
        pos: u64,
        length: u64,
        target: &mut dyn Writable,
        buffer_size: u64,
    ) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.copy(length, target, buffer_size);
        self.to(pos_before)?;
        result
    }

    /// Copies data from the current position to a target at a specific position to a specific position.
    ///
    /// This executes the [`copy_to`][Readable::copy_to] method at `pos` and then returns to the original position.
    fn copy_to_at(
        &mut self,
        pos: u64,
        target_pos: u64,
        length: u64,
        target: &mut dyn Writable,
        buffer_size: u64,
    ) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.copy_to(target_pos, length, target, buffer_size);
        self.to(pos_before)?;
        result
    }

    /// Reads bytes at a specific position.
    ///
    /// This executes the [`read_bytes`][Readable::read_bytes] method at `pos` and then returns to the original position.
    fn read_bytes_at(&mut self, pos: u64, len: u64) -> Result<Vec<u8>> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_bytes(len);
        self.to(pos_before)?;
        result
    }

    /// Reads an UTF-8-encoded string at a specific position.
    ///
    /// This executes the [`read_utf8`][Readable::read_utf8] method at `pos` and then returns to the original position.
    fn read_utf8_at(&mut self, pos: u64, len: u64) -> Result<String> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_utf8(len);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 8-bit integer at a specific position.
    ///
    /// This executes the [`read_u8`][Readable::read_u8] method at `pos` and then returns to the original position.
    fn read_u8_at(&mut self, pos: u64) -> Result<u8> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u8();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 16-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_u16le`][Readable::read_u16le] method at `pos` and then returns to the original position.
    fn read_u16le_at(&mut self, pos: u64) -> Result<u16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u16le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 16-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_u16be`][Readable::read_u16be] method at `pos` and then returns to the original position.
    fn read_u16be_at(&mut self, pos: u64) -> Result<u16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u16be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 32-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_u32le`][Readable::read_u32le] method at `pos` and then returns to the original position.
    fn read_u32le_at(&mut self, pos: u64) -> Result<u32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u32le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 32-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_u32be`][Readable::read_u32be] method at `pos` and then returns to the original position.
    fn read_u32be_at(&mut self, pos: u64) -> Result<u32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u32be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 64-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_u64le`][Readable::read_u64le] method at `pos` and then returns to the original position.
    fn read_u64le_at(&mut self, pos: u64) -> Result<u64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u64le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 64-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_u64be`][Readable::read_u64be] method at `pos` and then returns to the original position.
    fn read_u64be_at(&mut self, pos: u64) -> Result<u64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u64be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 128-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_u128le`][Readable::read_u128le] method at `pos` and then returns to the original position.
    fn read_u128le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u128le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 128-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_u128be`][Readable::read_u128be] method at `pos` and then returns to the original position.
    fn read_u128be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u128be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 7-bit variable-length integer at a specific position.
    ///
    /// This executes the [`read_vu7`][Readable::read_vu7] method at `pos` and then returns to the original position.
    fn read_vu7_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu7();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 7-bit variable-length integer in reversed byte order at a specific position.
    ///
    /// This executes the [`read_vu7r`][Readable::read_vu7r] method at `pos` and then returns to the original position.
    fn read_vu7r_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu7r();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 15-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu15le`][Readable::read_vu15le] method at `pos` and then returns to the original position.
    fn read_vu15le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 15-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu15be`][Readable::read_vu15be] method at `pos` and then returns to the original position.
    fn read_vu15be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu15ler`][Readable::read_vu15ler] method at `pos` and then returns to the original position.
    fn read_vu15ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15ler();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu15ber`][Readable::read_vu15ber] method at `pos` and then returns to the original position.
    fn read_vu15ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15ber();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 31-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu31le`][Readable::read_vu31le] method at `pos` and then returns to the original position.
    fn read_vu31le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 31-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu31be`][Readable::read_vu31be] method at `pos` and then returns to the original position.
    fn read_vu31be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu31ler`][Readable::read_vu31ler] method at `pos` and then returns to the original position.
    fn read_vu31ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31ler();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu31ber`][Readable::read_vu31ber] method at `pos` and then returns to the original position.
    fn read_vu31ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31ber();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 63-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu63le`][Readable::read_vu63le] method at `pos` and then returns to the original position.
    fn read_vu63le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 63-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu63be`][Readable::read_vu63be] method at `pos` and then returns to the original position.
    fn read_vu63be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu63ler`][Readable::read_vu63ler] method at `pos` and then returns to the original position.
    fn read_vu63ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63ler();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 63-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu63ber`][Readable::read_vu63ber] method at `pos` and then returns to the original position.
    fn read_vu63ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63ber();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 127-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu127le`][Readable::read_vu127le] method at `pos` and then returns to the original position.
    fn read_vu127le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127le();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 127-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu127be`][Readable::read_vu127be] method at `pos` and then returns to the original position.
    fn read_vu127be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127be();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu127ler`][Readable::read_vu127ler] method at `pos` and then returns to the original position.
    fn read_vu127ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127ler();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vu127ber`][Readable::read_vu127ber] method at `pos` and then returns to the original position.
    fn read_vu127ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127ber();
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_uxle`][Readable::read_uxle] method at `pos` and then returns to the original position.
    fn read_uxle_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_uxle(size);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_uxbe`][Readable::read_uxbe] method at `pos` and then returns to the original position.
    fn read_uxbe_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_uxbe(size);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vuxle`][Readable::read_vuxle] method at `pos` and then returns to the original position.
    fn read_vuxle_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxle(size);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vuxbe`][Readable::read_vuxbe] method at `pos` and then returns to the original position.
    fn read_vuxbe_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxbe(size);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vuxler`][Readable::read_vuxler] method at `pos` and then returns to the original position.
    fn read_vuxler_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxler(size);
        self.to(pos_before)?;
        result
    }

    /// Reads an unsigned variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vuxber`][Readable::read_vuxber] method at `pos` and then returns to the original position.
    fn read_vuxber_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxber(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 8-bit integer at a specific position.
    ///
    /// This executes the [`read_i8`][Readable::read_i8] method at `pos` and then returns to the original position.
    fn read_i8_at(&mut self, pos: u64) -> Result<i8> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i8();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 16-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_i16le`][Readable::read_i16le] method at `pos` and then returns to the original position.
    fn read_i16le_at(&mut self, pos: u64) -> Result<i16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i16le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 16-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_i16be`][Readable::read_i16be] method at `pos` and then returns to the original position.
    fn read_i16be_at(&mut self, pos: u64) -> Result<i16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i16be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 32-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_i32le`][Readable::read_i32le] method at `pos` and then returns to the original position.
    fn read_i32le_at(&mut self, pos: u64) -> Result<i32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i32le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 32-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_i32be`][Readable::read_i32be] method at `pos` and then returns to the original position.
    fn read_i32be_at(&mut self, pos: u64) -> Result<i32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i32be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 64-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_i64le`][Readable::read_i64le] method at `pos` and then returns to the original position.
    fn read_i64le_at(&mut self, pos: u64) -> Result<i64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i64le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 64-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_i64be`][Readable::read_i64be] method at `pos` and then returns to the original position.
    fn read_i64be_at(&mut self, pos: u64) -> Result<i64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i64be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 128-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_i128le`][Readable::read_i128le] method at `pos` and then returns to the original position.
    fn read_i128le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i128le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 128-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_i128be`][Readable::read_i128be] method at `pos` and then returns to the original position.
    fn read_i128be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i128be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 7-bit variable-length integer at a specific position.
    ///
    /// This executes the [`read_vi7`][Readable::read_vi7] method at `pos` and then returns to the original position.
    fn read_vi7_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi7();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 7-bit variable-length integer in reversed byte order at a specific position.
    ///
    /// This executes the [`read_vi7r`][Readable::read_vi7r] method at `pos` and then returns to the original position.
    fn read_vi7r_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi7r();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 15-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi15le`][Readable::read_vi15le] method at `pos` and then returns to the original position.
    fn read_vi15le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 15-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi15be`][Readable::read_vi15be] method at `pos` and then returns to the original position.
    fn read_vi15be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi15ler`][Readable::read_vi15ler] method at `pos` and then returns to the original position.
    fn read_vi15ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15ler();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi15ber`][Readable::read_vi15ber] method at `pos` and then returns to the original position.
    fn read_vi15ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15ber();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 31-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi31le`][Readable::read_vi31le] method at `pos` and then returns to the original position.
    fn read_vi31le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 31-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi31be`][Readable::read_vi31be] method at `pos` and then returns to the original position.
    fn read_vi31be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi31ler`][Readable::read_vi31ler] method at `pos` and then returns to the original position.
    fn read_vi31ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31ler();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi31ber`][Readable::read_vi31ber] method at `pos` and then returns to the original position.
    fn read_vi31ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31ber();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 63-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi63le`][Readable::read_vi63le] method at `pos` and then returns to the original position.
    fn read_vi63le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 63-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi63be`][Readable::read_vi63be] method at `pos` and then returns to the original position.
    fn read_vi63be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi63ler`][Readable::read_vi63ler] method at `pos` and then returns to the original position.
    fn read_vi63ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63ler();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 63-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi63ber`][Readable::read_vi63ber] method at `pos` and then returns to the original position.
    fn read_vi63ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63ber();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 127-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi127le`][Readable::read_vi127le] method at `pos` and then returns to the original position.
    fn read_vi127le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127le();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 127-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi127be`][Readable::read_vi127be] method at `pos` and then returns to the original position.
    fn read_vi127be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127be();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi127ler`][Readable::read_vi127ler] method at `pos` and then returns to the original position.
    fn read_vi127ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127ler();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vi127ber`][Readable::read_vi127ber] method at `pos` and then returns to the original position.
    fn read_vi127ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127ber();
        self.to(pos_before)?;
        result
    }

    /// Reads a signed integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_ixle`][Readable::read_ixle] method at `pos` and then returns to the original position.
    fn read_ixle_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_ixle(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_ixbe`][Readable::read_ixbe] method at `pos` and then returns to the original position.
    fn read_ixbe_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_ixbe(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vixle`][Readable::read_vixle] method at `pos` and then returns to the original position.
    fn read_vixle_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixle(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vixbe`][Readable::read_vixbe] method at `pos` and then returns to the original position.
    fn read_vixbe_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixbe(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`read_vixler`][Readable::read_vixler] method at `pos` and then returns to the original position.
    fn read_vixler_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixler(size);
        self.to(pos_before)?;
        result
    }

    /// Reads a signed variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`read_vixber`][Readable::read_vixber] method at `pos` and then returns to the original position.
    fn read_vixber_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixber(size);
        self.to(pos_before)?;
        result
    }

    /// Copies data from the current position to a target.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut src = dh::data::read(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let mut target = dh::data::write_new(8);
    ///
    /// src.copy(4, &mut target, 1024).unwrap();
    /// src.rewind().unwrap();
    /// src.copy(4, &mut target, 1024).unwrap();
    ///
    /// let data = dh::data::close(target);
    /// assert_eq!(data, vec![0, 1, 2, 3, 0, 1, 2, 3]);
    /// ```
    fn copy(&mut self, length: u64, target: &mut dyn Writable, buffer_size: u64) -> Result<()> {
        let mut buf = vec![0; buffer_size as usize];
        let mut remaining = length;

        while remaining > 0 {
            let read = min(remaining, buffer_size);
            if buffer_size > read {
                buf.resize(read as usize, 0);
            }
            self.read_exact(&mut buf[..read as usize])?;
            target.write_all(&buf[..read as usize])?;
            remaining -= read;
        }
        Ok(())
    }

    /// Copies data from the current position to a target to a specific position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut src = dh::data::read(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let mut target = dh::data::write_new(8);
    ///
    /// src.jump(1).unwrap();
    /// src.copy_to(2, 4, &mut target, 1024).unwrap();
    ///
    /// let data = dh::data::close(target);
    /// assert_eq!(data, vec![0, 0, 1, 2, 3, 4, 0, 0]);
    /// ```
    fn copy_to(
        &mut self,
        target_pos: u64,
        length: u64,
        target: &mut dyn Writable,
        buffer_size: u64,
    ) -> Result<()> {
        let target_pos_before = target.stream_position()?;
        target.to(target_pos)?;
        let result = self.copy(length, target, buffer_size);
        target.to(target_pos_before)?;
        result
    }

    /// Reads bytes at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0, 1, 2, 3, 4, 5]);
    /// reader.jump(2).unwrap();
    ///
    /// let bytes = reader.read_bytes(3).unwrap();
    /// assert_eq!(bytes, vec![2, 3, 4]);
    /// ```
    fn read_bytes(&mut self, length: u64) -> Result<Vec<u8>> {
        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads an UTF-8-encoded string at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    ///
    /// let string = reader.read_utf8(5).unwrap();
    /// assert_eq!(string, "Hello");
    /// ```
    fn read_utf8(&mut self, length: u64) -> Result<String> {
        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf)?;
        Ok(match String::from_utf8(buf) {
            Ok(str) => str,
            Err(_) => return Err(ErrorKind::InvalidData.into()),
        })
    }

    /// Reads an unsigned 8-bit integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x48]);
    ///
    /// let byte = reader.read_u8().unwrap();
    /// assert_eq!(byte, 0x48);
    /// ```
    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_uxle(1)? as u8)
    }

    /// Reads an unsigned 16-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02]);
    ///
    /// let num = reader.read_u16le().unwrap();
    /// assert_eq!(num, 0x02_01);
    /// ```
    fn read_u16le(&mut self) -> Result<u16> {
        Ok(self.read_uxle(2)? as u16)
    }

    /// Reads an unsigned 16-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02]);
    ///
    /// let num = reader.read_u16be().unwrap();
    /// assert_eq!(num, 0x01_02);
    /// ```
    fn read_u16be(&mut self) -> Result<u16> {
        Ok(self.read_uxbe(2)? as u16)
    }

    /// Reads an unsigned 32-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04]);
    ///
    /// let num = reader.read_u32le().unwrap();
    /// assert_eq!(num, 0x04_03_02_01);
    /// ```
    fn read_u32le(&mut self) -> Result<u32> {
        Ok(self.read_uxle(4)? as u32)
    }

    /// Reads an unsigned 32-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04]);
    ///
    /// let num = reader.read_u32be().unwrap();
    /// assert_eq!(num, 0x01_02_03_04);
    /// ```
    fn read_u32be(&mut self) -> Result<u32> {
        Ok(self.read_uxbe(4)? as u32)
    }

    /// Reads an unsigned 64-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    ///
    /// let num = reader.read_u64le().unwrap();
    /// assert_eq!(num, 0x08_07_06_05_04_03_02_01);
    /// ```
    fn read_u64le(&mut self) -> Result<u64> {
        Ok(self.read_uxle(8)? as u64)
    }

    /// Reads an unsigned 64-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    ///
    /// let num = reader.read_u64be().unwrap();
    /// assert_eq!(num, 0x01_02_03_04_05_06_07_08);
    /// ```
    fn read_u64be(&mut self) -> Result<u64> {
        Ok(self.read_uxbe(8)? as u64)
    }

    /// Reads an unsigned 128-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    ///
    /// let num = reader.read_u128le().unwrap();
    ///
    /// assert_eq!(num, 0x10_0f_0e_0d_0c_0b_0a_09_08_07_06_05_04_03_02_01);
    /// ```
    fn read_u128le(&mut self) -> Result<u128> {
        self.read_uxle(16)
    }

    /// Reads an unsigned 128-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    ///
    /// let num = reader.read_u128be().unwrap();
    ///
    /// assert_eq!(num, 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10);
    /// ```
    fn read_u128be(&mut self) -> Result<u128> {
        self.read_uxbe(16)
    }

    /// Reads an unsigned 7-bit variable-length integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vu7().unwrap();
    /// assert_eq!(num, 0b1000001_0010000);
    /// ```
    fn read_vu7(&mut self) -> Result<u128> {
        self.read_vuxle(1)
    }

    /// Reads an unsigned 7-bit variable-length integer in reversed byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vu7r().unwrap();
    /// assert_eq!(num, 0b0010000_1000001);
    /// ```
    fn read_vu7r(&mut self) -> Result<u128> {
        self.read_vuxler(1)
    }

    /// Reads an unsigned 15-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10101010, 0b11000001, 0b10101010, 0b01000001]);
    ///
    /// let num = reader.read_vu15le().unwrap();
    /// assert_eq!(num, 0b100000110101010_100000110101010);
    /// ```
    fn read_vu15le(&mut self) -> Result<u128> {
        self.read_vuxle(2)
    }

    /// Reads an unsigned 15-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b11000001, 0b10101010, 0b01000001, 0b10101010]);
    ///
    /// let num = reader.read_vu15be().unwrap();
    /// assert_eq!(num, 0b100000110101010_100000110101010);
    /// ```
    fn read_vu15be(&mut self) -> Result<u128> {
        self.read_vuxbe(2)
    }

    /// Reads an unsigned 15-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10101010, 0b11001001, 0b10101010, 0b01000001]);
    ///
    /// let num = reader.read_vu15ler().unwrap();
    /// assert_eq!(num, 0b100100110101010_100000110101010);
    /// ```
    fn read_vu15ler(&mut self) -> Result<u128> {
        self.read_vuxler(2)
    }

    /// Reads an unsigned 15-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b11001001, 0b10101010, 0b01000001, 0b10101010]);
    ///
    /// let num = reader.read_vu15ber().unwrap();
    /// assert_eq!(num, 0b100100110101010_100000110101010);
    /// ```
    fn read_vu15ber(&mut self) -> Result<u128> {
        self.read_vuxber(2)
    }

    /// Reads an unsigned 31-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10101010, 0b10000001, 0b10101010, 0b11000001, 0b10101010, 0b10000001, 0b10101010, 0b01000001]);
    ///
    /// let num = reader.read_vu31le().unwrap();
    /// assert_eq!(num, 0b1000001101010101000000110101010_1000001101010101000000110101010);
    /// ```
    fn read_vu31le(&mut self) -> Result<u128> {
        self.read_vuxle(4)
    }

    /// Reads an unsigned 31-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b11000001, 0b10101010, 0b10000001, 0b10101010, 0b01000001, 0b10101010, 0b10000001, 0b10101010]);
    ///
    /// let num = reader.read_vu31be().unwrap();
    /// assert_eq!(num, 0b1000001101010101000000110101010_1000001101010101000000110101010);
    /// ```
    fn read_vu31be(&mut self) -> Result<u128> {
        self.read_vuxbe(4)
    }

    /// Reads an unsigned 31-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10101010, 0b10000001, 0b10101010, 0b11000001, 0b10101010, 0b10000001, 0b10101010, 0b01000001]);
    ///
    /// let num = reader.read_vu31ler().unwrap();
    /// assert_eq!(num, 0b1000001101010101000000110101010_1000001101010101000000110101010);
    /// ```
    fn read_vu31ler(&mut self) -> Result<u128> {
        self.read_vuxler(4)
    }

    /// Reads an unsigned 31-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b11000001, 0b10101010, 0b10000001, 0b10101010, 0b01000001, 0b10101010, 0b10000001, 0b10101010]);
    ///
    /// let num = reader.read_vu31ber().unwrap();
    /// assert_eq!(num, 0b1000001101010101000000110101010_1000001101010101000000110101010);
    /// ```
    fn read_vu31ber(&mut self) -> Result<u128> {
        self.read_vuxber(4)
    }

    /// Reads an unsigned 63-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31le`][Readable::read_vu31le] but with 63 bits.
    fn read_vu63le(&mut self) -> Result<u128> {
        self.read_vuxle(8)
    }

    /// Reads an unsigned 63-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31be`][Readable::read_vu31be] but with 63 bits.
    fn read_vu63be(&mut self) -> Result<u128> {
        self.read_vuxbe(8)
    }

    /// Reads an unsigned 63-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ler`][Readable::read_vu31ler] but with 63 bits.
    fn read_vu63ler(&mut self) -> Result<u128> {
        self.read_vuxler(8)
    }

    /// Reads an unsigned 63-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ber`][Readable::read_vu31ber] but with 63 bits.
    fn read_vu63ber(&mut self) -> Result<u128> {
        self.read_vuxber(8)
    }

    /// Reads an unsigned 127-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31le`][Readable::read_vu31le] but with 127 bits.
    fn read_vu127le(&mut self) -> Result<u128> {
        self.read_vuxle(16)
    }

    /// Reads an unsigned 127-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31be`][Readable::read_vu31be] but with 127 bits.
    fn read_vu127be(&mut self) -> Result<u128> {
        self.read_vuxbe(16)
    }

    /// Reads an unsigned 127-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ler`][Readable::read_vu31ler] but with 127 bits.
    fn read_vu127ler(&mut self) -> Result<u128> {
        self.read_vuxler(16)
    }

    /// Reads an unsigned 127-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ber`][Readable::read_vu31ber] but with 127 bits.
    fn read_vu127ber(&mut self) -> Result<u128> {
        self.read_vuxber(16)
    }

    /// Reads an unsigned integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03]);
    ///
    /// let num = reader.read_uxle(3).unwrap();
    /// assert_eq!(num, 0x03_02_01);
    /// ```
    fn read_uxle(&mut self, size: u8) -> Result<u128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << (i * 8);
        }

        Ok(result)
    }

    /// Reads an unsigned integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0x01, 0x02, 0x03]);
    ///
    /// let num = reader.read_uxbe(3).unwrap();
    /// assert_eq!(num, 0x01_02_03);
    /// ```
    fn read_uxbe(&mut self, size: u8) -> Result<u128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }

    /// Reads an unsigned variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vuxle(1).unwrap();
    /// assert_eq!(num, 0b1000001_0010000);
    /// ```
    fn read_vuxle(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vux(&mut |s| self.read_uxle(s), size)?.0)
    }

    /// Reads an unsigned variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vuxbe(1).unwrap();
    /// assert_eq!(num, 0b1000001_0010000);
    /// ```
    fn read_vuxbe(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vux(&mut |s| self.read_uxbe(s), size)?.0)
    }

    /// Reads an unsigned variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vuxler(1).unwrap();
    /// assert_eq!(num, 0b0010000_1000001);
    /// ```
    fn read_vuxler(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vuxr(&mut |s| self.read_uxle(s), size)?.0)
    }

    /// Reads an unsigned variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10010000, 0b01000001]);
    ///
    /// let num = reader.read_vuxber(1).unwrap();
    /// assert_eq!(num, 0b0010000_1000001);
    /// ```
    fn read_vuxber(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vuxr(&mut |s| self.read_uxbe(s), size)?.0)
    }

    /// Reads a signed 8-bit integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i8::MIN.to_le_bytes().to_vec());
    ///
    /// let byte = reader.read_i8().unwrap();
    /// assert_eq!(byte, i8::MIN);
    /// ```
    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_ixle(1)? as i8)
    }

    /// Reads a signed 16-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i16::MIN.to_le_bytes().to_vec());
    ///
    /// let num = reader.read_i16le().unwrap();
    /// assert_eq!(num, i16::MIN);
    /// ```
    fn read_i16le(&mut self) -> Result<i16> {
        Ok(self.read_ixle(2)? as i16)
    }

    /// Reads a signed 16-bit integer in big-endian byte order at the current position.  
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i16::MIN.to_be_bytes().to_vec());
    ///
    /// let num = reader.read_i16be().unwrap();
    /// assert_eq!(num, i16::MIN);
    /// ```      
    fn read_i16be(&mut self) -> Result<i16> {
        Ok(self.read_ixbe(2)? as i16)
    }

    /// Reads a signed 32-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i32::MIN.to_le_bytes().to_vec());
    ///
    /// let num = reader.read_i32le().unwrap();
    /// assert_eq!(num, i32::MIN);
    /// ```
    fn read_i32le(&mut self) -> Result<i32> {
        Ok(self.read_ixle(4)? as i32)
    }

    /// Reads a signed 32-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i32::MIN.to_be_bytes().to_vec());
    ///
    /// let num = reader.read_i32be().unwrap();
    /// assert_eq!(num, i32::MIN);
    /// ```
    fn read_i32be(&mut self) -> Result<i32> {
        Ok(self.read_ixbe(4)? as i32)
    }

    /// Reads a signed 64-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i64::MIN.to_le_bytes().to_vec());
    ///
    /// let num = reader.read_i64le().unwrap();
    /// assert_eq!(num, i64::MIN);
    /// ```
    fn read_i64le(&mut self) -> Result<i64> {
        Ok(self.read_ixle(8)? as i64)
    }

    /// Reads a signed 64-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i64::MIN.to_be_bytes().to_vec());
    ///
    /// let num = reader.read_i64be().unwrap();
    /// assert_eq!(num, i64::MIN);
    /// ```
    fn read_i64be(&mut self) -> Result<i64> {
        Ok(self.read_ixbe(8)? as i64)
    }

    /// Reads a signed 128-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i128::MIN.to_le_bytes().to_vec());
    ///
    /// let num = reader.read_i128le().unwrap();
    /// assert_eq!(num, i128::MIN);
    /// ```
    fn read_i128le(&mut self) -> Result<i128> {
        self.read_ixle(16)
    }

    /// Reads a signed 128-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i128::MIN.to_be_bytes().to_vec());
    ///
    /// let num = reader.read_i128be().unwrap();
    /// assert_eq!(num, i128::MIN);
    /// ```
    fn read_i128be(&mut self) -> Result<i128> {
        self.read_ixbe(16)
    }

    /// Reads a signed 7-bit variable-length integer at the current position.
    ///
    /// This works like [`read_vu7`][Readable::read_vu7] but for signed integers.
    fn read_vi7(&mut self) -> Result<i128> {
        self.read_vixle(1)
    }

    /// Reads a signed 7-bit variable-length integer in reversed byte order at the current position.
    ///
    /// This works like [`read_vu7r`][Readable::read_vu7r] but for signed integers.
    fn read_vi7r(&mut self) -> Result<i128> {
        self.read_vixler(1)
    }

    /// Reads a signed 15-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu15le`][Readable::read_vu15le] but for signed integers.
    fn read_vi15le(&mut self) -> Result<i128> {
        self.read_vixle(2)
    }

    /// Reads a signed 15-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu15be`][Readable::read_vu15be] but for signed integers.
    fn read_vi15be(&mut self) -> Result<i128> {
        self.read_vixbe(2)
    }

    /// Reads a signed 15-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu15ler`][Readable::read_vu15ler] but for signed integers.
    fn read_vi15ler(&mut self) -> Result<i128> {
        self.read_vixler(2)
    }

    /// Reads a signed 15-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu15ber`][Readable::read_vu15ber] but for signed integers.
    fn read_vi15ber(&mut self) -> Result<i128> {
        self.read_vixber(2)
    }

    /// Reads a signed 31-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31le`][Readable::read_vu31le] but for signed integers.
    fn read_vi31le(&mut self) -> Result<i128> {
        self.read_vixle(4)
    }

    /// Reads a signed 31-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31be`][Readable::read_vu31be] but for signed integers.
    fn read_vi31be(&mut self) -> Result<i128> {
        self.read_vixbe(4)
    }

    /// Reads a signed 31-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ler`][Readable::read_vu31ler] but for signed integers.
    fn read_vi31ler(&mut self) -> Result<i128> {
        self.read_vixler(4)
    }

    /// Reads a signed 31-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu31ber`][Readable::read_vu31ber] but for signed integers.
    fn read_vi31ber(&mut self) -> Result<i128> {
        self.read_vixber(4)
    }

    /// Reads a signed 63-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu63le`][Readable::read_vu63le] but for signed integers.
    fn read_vi63le(&mut self) -> Result<i128> {
        self.read_vixle(8)
    }

    /// Reads a signed 63-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu63be`][Readable::read_vu63be] but for signed integers.
    fn read_vi63be(&mut self) -> Result<i128> {
        self.read_vixbe(8)
    }

    /// Reads a signed 63-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu63ler`][Readable::read_vu63ler] but for signed integers.
    fn read_vi63ler(&mut self) -> Result<i128> {
        self.read_vixler(8)
    }

    /// Reads a signed 63-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu63ber`][Readable::read_vu63ber] but for signed integers.
    fn read_vi63ber(&mut self) -> Result<i128> {
        self.read_vixber(8)
    }

    /// Reads a signed 127-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`read_vu127le`][Readable::read_vu127le] but for signed integers.
    fn read_vi127le(&mut self) -> Result<i128> {
        self.read_vixle(16)
    }

    /// Reads a signed 127-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`read_vu127be`][Readable::read_vu127be] but for signed integers.
    fn read_vi127be(&mut self) -> Result<i128> {
        self.read_vixbe(16)
    }

    /// Reads a signed 127-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`read_vu127ler`][Readable::read_vu127ler] but for signed integers.
    fn read_vi127ler(&mut self) -> Result<i128> {
        self.read_vixler(16)
    }

    /// Reads a signed 127-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`read_vu127ber`][Readable::read_vu127ber] but for signed integers.
    fn read_vi127ber(&mut self) -> Result<i128> {
        self.read_vixber(16)
    }

    /// Reads a signed integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i8::MIN.to_le_bytes().to_vec());
    ///
    /// let byte = reader.read_ixle(1).unwrap() as i8;
    /// assert_eq!(byte, i8::MIN);
    /// ```
    fn read_ixle(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << (i * 8);
        }

        Ok(result)
    }

    /// Reads a signed integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(i8::MIN.to_be_bytes().to_vec());
    ///
    /// let byte = reader.read_ixbe(1).unwrap() as i8;
    /// assert_eq!(byte, i8::MIN);
    /// ```
    fn read_ixbe(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }

    /// Reads a signed variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b10000000, 0b01000000]);
    ///
    /// let num = reader.read_vixle(1).unwrap();
    /// assert_eq!(num, -8192);
    /// ```
    fn read_vixle(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxle(s);
        let result = parse_vux(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        Ok(unsigned_to_signed(unsigned, length, size))
    }

    /// Reads a signed variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b01100000, 0b00000000]);
    ///
    /// let num = reader.read_vixbe(2).unwrap();
    /// assert_eq!(num, -8192);
    /// ```
    fn read_vixbe(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxbe(s);
        let result = parse_vux(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        Ok(unsigned_to_signed(unsigned, length, size))
    }

    /// Reads a signed variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b00000000, 0b01100000]);
    ///
    /// let num = reader.read_vixler(2).unwrap();
    /// assert_eq!(num, -8192);
    /// ```
    fn read_vixler(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxle(s);
        let result = parse_vuxr(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        Ok(unsigned_to_signed(unsigned, length, size))
    }

    /// Reads a signed variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0b01100000, 0b00000000]);
    ///
    /// let num = reader.read_vixber(2).unwrap();
    /// assert_eq!(num, -8192);
    /// ```
    fn read_vixber(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxbe(s);
        let result = parse_vuxr(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        Ok(unsigned_to_signed(unsigned, length, size))
    }
}
