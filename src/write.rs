use std::io::{Result, Write};

use crate::{limit_w, DataType, Seekable, Source, WLimited};

fn signed_to_unsigned(num: i128, size: u8) -> u128 {
    if size == 16 {
        // can't use 1 << 128 because it would overflow
        return num as u128;
    }
    let mask = (1 << (size * 8)) - 1;
    (num & mask) as u128
}

fn signed_to_unsigned_vi(num: i128, size: u8) -> u128 {
    if size == 16 {
        // can't use 1 << 128 because it would overflow
        return num as u128;
    }
    let mask = (1 << (size * 8 - 1)) - 1;
    (num & mask) as u128
}

fn serialize_vuxle(size: u8, num: u128, be: bool, rev: bool) -> Vec<u8> {
    let mut num = num;
    let mut buf = Vec::new();
    let shift = (8 * size) - 1;
    let max_size = 1 << shift;
    let mask = max_size - 1;
    while num >= max_size {
        buf.push(num & mask);
        num >>= shift;
    }
    buf.push(num);
    if rev {
        buf.reverse();
    }

    let last_index = buf.len() - 1;
    buf.iter_mut().take(last_index).for_each(|x| {
        *x |= max_size;
    });
    let buf = buf
        .iter()
        .flat_map(|&x| {
            if be {
                let mut bytes = x.to_be_bytes().to_vec();
                bytes.drain(0..(bytes.len() - size as usize));
                return bytes;
            }

            let mut bytes = x.to_le_bytes().to_vec();
            bytes.truncate(size as usize);
            bytes
        })
        .collect::<Vec<u8>>();
    buf
}

/// Provides methods to write data to a target.
///
/// Although the trait can be implemented for any type that implements [`Write`] and [`Seekable`],
/// for most cases the [internal implementations](#implementors) are recommended.
pub trait Writable<'a>
where
    Self: Write + Seekable,
{
    /// An internal method to get the reader as a trait object.
    /// Yes, this is kinda nonsense, but Rust forces me into that.
    ///
    /// ### How you implement it
    ///
    /// ```ignore
    /// fn as_trait(&mut self) -> &mut dyn Writable<'a> {
    ///     self
    /// }
    /// ```
    fn as_trait(&mut self) -> &mut dyn Writable<'a>;

    /// Borrows the write source.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let source = reader.source();
    /// match source {
    ///     Vec(source) => assert_eq!(source, &mut vec![0, 1, 2, 3, 4, 5, 6, 7]),
    ///     _ => unreachable!(),
    /// }
    /// ```
    fn source(&mut self) -> Source;

    /// Closes the reader and can return the source if it was moved or references it.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::write(vec![]);
    /// // do something with the writer
    /// reader.close().unwrap(); // if the writer goes out of scope, this happens automatically
    /// ```
    fn close(self) -> Result<Option<DataType<'a>>>;

    /// Limits the space the writer can write to.
    ///
    /// The writer will only be able to write from `pos` to `pos + length`.
    ///
    /// **Note:** The writer will automatically jump to the start of the limit here.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let mut limited = writer.limit(2, 4).unwrap();
    ///
    /// let size = limited.size().unwrap();
    /// limited.write_bytes(&vec![5, 4, 3, 2]).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0, 1, 5, 4, 3, 2, 6, 7]);
    /// ```
    fn limit(&'a mut self, pos: u64, length: u64) -> Result<WLimited<'a>> {
        limit_w(self.as_trait(), pos, length)
    }

    /// Writes bytes at a specific position.
    ///
    /// This executes the [`write_bytes`][Writable::write_bytes] method at `pos` and then returns to the original position.
    fn write_bytes_at(&mut self, pos: u64, buf: &[u8]) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_bytes(buf)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an UTF-8-encoded string at a specific position.
    ///
    /// This executes the [`write_utf8`][Writable::write_utf8] method at `pos` and then returns to the original position.
    fn write_utf8_at(&mut self, pos: u64, s: &String) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_utf8(s)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 8-bit integer at a specific position.
    ///
    /// This executes the [`write_u8`][Writable::write_u8] method at `pos` and then returns to the original position.
    fn write_u8_at(&mut self, pos: u64, num: u8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 16-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_u16le`][Writable::write_u16le] method at `pos` and then returns to the original position.
    fn write_u16le_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 16-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_u16be`][Writable::write_u16be] method at `pos` and then returns to the original position.
    fn write_u16be_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 32-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_u32le`][Writable::write_u32le] method at `pos` and then returns to the original position.
    fn write_u32le_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 32-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_u32be`][Writable::write_u32be] method at `pos` and then returns to the original position.
    fn write_u32be_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 64-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_u64le`][Writable::write_u64le] method at `pos` and then returns to the original position.
    fn write_u64le_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 64-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_u64be`][Writable::write_u64be] method at `pos` and then returns to the original position.
    fn write_u64be_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 128-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_u128le`][Writable::write_u128le] method at `pos` and then returns to the original position.
    fn write_u128le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 128-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_u128be`][Writable::write_u128be] method at `pos` and then returns to the original position.
    fn write_u128be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 7-bit variable-length integer at a specific position.
    ///
    /// This executes the [`write_vu7`][Writable::write_vu7] method at `pos` and then returns to the original position.
    fn write_vu7_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 7-bit variable-length integer in reversed byte order at a specific position.
    ///
    /// This executes the [`write_vu7r`][Writable::write_vu7r] method at `pos` and then returns to the original position.
    fn write_vu7r_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7r(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu15le`][Writable::write_vu15le] method at `pos` and then returns to the original position.
    fn write_vu15le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu15be`][Writable::write_vu15be] method at `pos` and then returns to the original position.
    fn write_vu15be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu15ler`][Writable::write_vu15ler] method at `pos` and then returns to the original position.
    fn write_vu15ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu15ber`][Writable::write_vu15ber] method at `pos` and then returns to the original position.
    fn write_vu15ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu31le`][Writable::write_vu31le] method at `pos` and then returns to the original position.
    fn write_vu31le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu31be`][Writable::write_vu31be] method at `pos` and then returns to the original position.
    fn write_vu31be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu31ler`][Writable::write_vu31ler] method at `pos` and then returns to the original position.
    fn write_vu31ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu31ber`][Writable::write_vu31ber] method at `pos` and then returns to the original position.
    fn write_vu31ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu63le`][Writable::write_vu63le] method at `pos` and then returns to the original position.
    fn write_vu63le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu63be`][Writable::write_vu63be] method at `pos` and then returns to the original position.
    fn write_vu63be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu63ler`][Writable::write_vu63ler] method at `pos` and then returns to the original position.
    fn write_vu63ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu63ber`][Writable::write_vu63ber] method at `pos` and then returns to the original position.
    fn write_vu63ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu127le`][Writable::write_vu127le] method at `pos` and then returns to the original position.
    fn write_vu127le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu127be`][Writable::write_vu127be] method at `pos` and then returns to the original position.
    fn write_vu127be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu127ler`][Writable::write_vu127ler] method at `pos` and then returns to the original position.
    fn write_vu127ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vu127ber`][Writable::write_vu127ber] method at `pos` and then returns to the original position.
    fn write_vu127ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_uxle`][Writable::write_uxle] method at `pos` and then returns to the original position.
    fn write_uxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_uxbe`][Writable::write_uxbe] method at `pos` and then returns to the original position.
    fn write_uxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vuxle`][Writable::write_vuxle] method at `pos` and then returns to the original position.
    fn write_vuxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vuxler`][Writable::write_vuxler] method at `pos` and then returns to the original position.
    fn write_vuxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vuxler`][Writable::write_vuxler] method at `pos` and then returns to the original position.
    fn write_vuxler_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vuxber`][Writable::write_vuxber] method at `pos` and then returns to the original position.
    fn write_vuxber_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 8-bit integer at a specific position.
    ///
    /// This executes the [`write_i8`][Writable::write_i8] method at `pos` and then returns to the original position.
    fn write_i8_at(&mut self, pos: u64, num: i8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 16-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_i16le`][Writable::write_i16le] method at `pos` and then returns to the original position.
    fn write_i16le_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 16-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_i16be`][Writable::write_i16be] method at `pos` and then returns to the original position.
    fn write_i16be_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 32-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_i32le`][Writable::write_i32le] method at `pos` and then returns to the original position.
    fn write_i32le_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 32-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_i32be`][Writable::write_i32be] method at `pos` and then returns to the original position.
    fn write_i32be_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 64-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_i64le`][Writable::write_i64le] method at `pos` and then returns to the original position.
    fn write_i64le_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 64-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_i64be`][Writable::write_i64be] method at `pos` and then returns to the original position.
    fn write_i64be_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 128-bit integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_i128le`][Writable::write_i128le] method at `pos` and then returns to the original position.
    fn write_i128le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 128-bit integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_i128be`][Writable::write_i128be] method at `pos` and then returns to the original position.
    fn write_i128be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 7-bit variable-length integer at a specific position.
    ///
    /// This executes the [`write_vi7`][Writable::write_vi7] method at `pos` and then returns to the original position.
    fn write_vi7_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi7(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 7-bit variable-length integer in reversed byte order at a specific position.
    ///
    /// This executes the [`write_vi7r`][Writable::write_vi7r] method at `pos` and then returns to the original position.
    fn write_vi7r_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi7r(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi15le`][Writable::write_vi15le] method at `pos` and then returns to the original position.
    fn write_vi15le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi15be`][Writable::write_vi15be] method at `pos` and then returns to the original position.
    fn write_vi15be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi15ler`][Writable::write_vi15ler] method at `pos` and then returns to the original position.
    fn write_vi15ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi15ber`][Writable::write_vi15ber] method at `pos` and then returns to the original position.
    fn write_vi15ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi31le`][Writable::write_vi31le] method at `pos` and then returns to the original position.
    fn write_vi31le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi31be`][Writable::write_vi31be] method at `pos` and then returns to the original position.
    fn write_vi31be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi31ler`][Writable::write_vi31ler] method at `pos` and then returns to the original position.
    fn write_vi31ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi31ber`][Writable::write_vi31ber] method at `pos` and then returns to the original position.
    fn write_vi31ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi63le`][Writable::write_vi63le] method at `pos` and then returns to the original position.
    fn write_vi63le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi63be`][Writable::write_vi63be] method at `pos` and then returns to the original position.
    fn write_vi63be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi63ler`][Writable::write_vi63ler] method at `pos` and then returns to the original position.
    fn write_vi63ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi127le`][Writable::write_vi127le] method at `pos` and then returns to the original position.
    fn write_vi127le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi127be`][Writable::write_vi127be] method at `pos` and then returns to the original position.
    fn write_vi127be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi127ler`][Writable::write_vi127ler] method at `pos` and then returns to the original position.
    fn write_vi127ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vi127ber`][Writable::write_vi127ber] method at `pos` and then returns to the original position.
    fn write_vi127ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_ixle`][Writable::write_ixle] method at `pos` and then returns to the original position.
    fn write_ixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_ixbe`][Writable::write_ixbe] method at `pos` and then returns to the original position.
    fn write_ixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vixle`][Writable::write_vixle] method at `pos` and then returns to the original position.
    fn write_vixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vixbe`][Writable::write_vixbe] method at `pos` and then returns to the original position.
    fn write_vixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in reversed little-endian byte order at a specific position.
    ///
    /// This executes the [`write_vixler`][Writable::write_vixler] method at `pos` and then returns to the original position.
    fn write_vixler_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in reversed big-endian byte order at a specific position.
    ///
    /// This executes the [`write_vixber`][Writable::write_vixber] method at `pos` and then returns to the original position.
    fn write_vixber_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes bytes at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(5);
    ///
    /// writer.write_bytes(&[1, 2, 3, 4, 5]).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![1, 2, 3, 4, 5]);
    /// ```
    fn write_bytes(&mut self, vec: &[u8]) -> Result<()> {
        self.write_all(vec)
    }

    /// Writes an UTF-8-encoded string at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(5);
    ///
    /// writer.write_utf8(&"Hello".to_string()).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]);
    /// ```
    fn write_utf8(&mut self, s: &String) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    /// Writes an unsigned 8-bit integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(1);
    ///
    /// writer.write_u8(0x48).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x48]);
    /// ```
    fn write_u8(&mut self, num: u8) -> Result<()> {
        self.write_all(&[num])
    }

    /// Writes an unsigned 16-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_u16le(0x02_01).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02]);
    /// ```
    fn write_u16le(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 16-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_u16be(0x01_02).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02]);
    /// ```
    fn write_u16be(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 32-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(4);
    ///
    /// writer.write_u32le(0x04_03_02_01).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04]);
    /// ```
    fn write_u32le(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 32-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(4);
    ///
    /// writer.write_u32be(0x01_02_03_04).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04]);
    /// ```
    fn write_u32be(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 64-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(8);
    ///
    /// writer.write_u64le(0x08_07_06_05_04_03_02_01).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    /// ```
    fn write_u64le(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 64-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(8);
    ///
    /// writer.write_u64be(0x01_02_03_04_05_06_07_08).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    /// ```
    fn write_u64be(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 128-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(16);
    ///
    /// writer.write_u128le(0x10_0f_0e_0d_0c_0b_0a_09_08_07_06_05_04_03_02_01).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    /// ```
    fn write_u128le(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 128-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(16);
    ///
    /// writer.write_u128be(0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f_10).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10]);
    /// ```
    fn write_u128be(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 7-bit variable-length integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu7(0xff).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu7().unwrap(), 0xff);
    /// ```
    fn write_vu7(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(1, num)
    }

    /// Writes an unsigned 7-bit variable-length integer in reversed byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu7r(0xff).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu7r().unwrap(), 0xff);
    /// ```
    fn write_vu7r(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(1, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu15le(0xff_ee).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu15le().unwrap(), 0xff_ee);
    /// ```
    fn write_vu15le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu15be(0xff_ee).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu15be().unwrap(), 0xff_ee);
    /// ```
    fn write_vu15be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu15ler(0xff_ee).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu15ler().unwrap(), 0xff_ee);
    /// ```
    fn write_vu15ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu15ber(0xff_ee).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu15ber().unwrap(), 0xff_ee);
    /// ```
    fn write_vu15ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(2, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu31le(0xff_ee_dd_cc).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu31le().unwrap(), 0xff_ee_dd_cc);
    /// ```
    fn write_vu31le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu31be(0xff_ee_dd_cc).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu31be().unwrap(), 0xff_ee_dd_cc);
    /// ```
    fn write_vu31be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu31ler(0xff_ee_dd_cc).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu31ler().unwrap(), 0xff_ee_dd_cc);
    /// ```
    fn write_vu31ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu31ber(0xff_ee_dd_cc).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu31ber().unwrap(), 0xff_ee_dd_cc);
    /// ```
    fn write_vu31ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(4, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu63le(0xff_ee_dd_cc_bb_aa_99_88).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu63le().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88);
    /// ```
    fn write_vu63le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu63be(0xff_ee_dd_cc_bb_aa_99_88).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu63be().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88);
    /// ```
    fn write_vu63be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu63ler(0xff_ee_dd_cc_bb_aa_99_88).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu63ler().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88);
    /// ```
    fn write_vu63ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu63ber(0xff_ee_dd_cc_bb_aa_99_88).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu63ber().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88);
    /// ```
    fn write_vu63ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(8, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu127le(0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu127le().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00);
    /// ```
    fn write_vu127le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu127be(0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu127be().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00);
    /// ```
    fn write_vu127be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu127ler(0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu127ler().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00);
    /// ```
    fn write_vu127ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vu127ber(0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vu127ber().unwrap(), 0xff_ee_dd_cc_bb_aa_99_88_77_66_55_44_33_22_11_00);
    /// ```
    fn write_vu127ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(16, num)
    }

    /// Writes an unsigned integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(3);
    ///
    /// writer.write_uxle(3, 0x01_02_03).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x03, 0x02, 0x01]);
    /// ```
    fn write_uxle(&mut self, size: u8, num: u128) -> Result<()> {
        if num >= 1 << (size * 8) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "number is too large for the given size",
            ));
        }
        let mut num = num;
        for _ in 0..size {
            self.write_all(&[num as u8])?;
            num >>= 8;
        }
        Ok(())
    }

    /// Writes an unsigned integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(3);
    ///
    /// writer.write_uxbe(3, 0x01_02_03).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0x01, 0x02, 0x03]);
    /// ```
    fn write_uxbe(&mut self, size: u8, num: u128) -> Result<()> {
        if num >= 1 << (size * 8) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "number is too large for the given size",
            ));
        }
        let mut num = num;
        for _ in 0..size {
            self.write_all(&[(num >> (8 * (size - 1))) as u8])?;
            num <<= 8;
        }
        Ok(())
    }

    /// Writes an unsigned variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vuxle(3, 0xff_ee_dd).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vuxle(3).unwrap(), 0xff_ee_dd);
    /// ```
    fn write_vuxle(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, false);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vuxbe(3, 0xff_ee_dd).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vuxbe(3).unwrap(), 0xff_ee_dd);
    /// ```
    fn write_vuxbe(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, false);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vuxler(3, 0xff_ee_dd).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vuxler(3).unwrap(), 0xff_ee_dd);
    /// ```
    fn write_vuxler(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, true);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vuxber(3, 0xff_ee_dd).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vuxber(3).unwrap(), 0xff_ee_dd);
    /// ```
    fn write_vuxber(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, true);
        self.write_all(&buf)
    }

    /// Writes a signed 8-bit integer at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(1);
    ///
    /// writer.write_i8(i8::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i8::MIN.to_le_bytes().to_vec());
    /// ```
    fn write_i8(&mut self, num: i8) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 16-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_i16le(i16::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i16::MIN.to_le_bytes().to_vec());
    /// ```
    fn write_i16le(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 16-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_i16be(i16::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i16::MIN.to_be_bytes().to_vec());
    /// ```
    fn write_i16be(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 32-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(4);
    ///
    /// writer.write_i32le(i32::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i32::MIN.to_le_bytes().to_vec());
    /// ```
    fn write_i32le(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 32-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(4);
    ///
    /// writer.write_i32be(i32::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i32::MIN.to_be_bytes().to_vec());
    /// ```
    fn write_i32be(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 64-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(8);
    ///
    /// writer.write_i64le(i64::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i64::MIN.to_le_bytes().to_vec());
    /// ```
    fn write_i64le(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 64-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(8);
    ///
    /// writer.write_i64be(i64::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i64::MIN.to_be_bytes().to_vec());
    /// ```
    fn write_i64be(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 128-bit integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(16);
    ///
    /// writer.write_i128le(i128::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i128::MIN.to_le_bytes().to_vec());
    /// ```
    fn write_i128le(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 128-bit integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(16);
    ///
    /// writer.write_i128be(i128::MIN).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, i128::MIN.to_be_bytes().to_vec());
    /// ```
    fn write_i128be(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 7-bit variable-length integer at the current position.
    ///
    /// This works like [`write_vu7`][Writable::write_vu7] but for signed integers.
    fn write_vi7(&mut self, num: i128) -> Result<()> {
        self.write_vixle(1, num)
    }

    /// Writes a signed 7-bit variable-length integer in reversed byte order at the current position.
    ///
    /// This works like [`write_vu7r`][Writable::write_vu7r] but for signed integers.
    fn write_vi7r(&mut self, num: i128) -> Result<()> {
        self.write_vixler(1, num)
    }

    /// Writes a signed 15-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`write_vu15le`][Writable::write_vu15le] but for signed integers.
    fn write_vi15le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`write_vu15be`][Writable::write_vu15be] but for signed integers.
    fn write_vi15be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`write_vu15ler`][Writable::write_vu15ler] but for signed integers.
    fn write_vi15ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`write_vu15ber`][Writable::write_vu15ber] but for signed integers.
    fn write_vi15ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(2, num)
    }

    /// Writes a signed 31-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`write_vu31le`][Writable::write_vu31le] but for signed integers.
    fn write_vi31le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`write_vu31be`][Writable::write_vu31be] but for signed integers.
    fn write_vi31be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`write_vu31ler`][Writable::write_vu31ler] but for signed integers.
    fn write_vi31ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`write_vu31ber`][Writable::write_vu31ber] but for signed integers.
    fn write_vi31ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(4, num)
    }

    /// Writes a signed 63-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`write_vu63le`][Writable::write_vu63le] but for signed integers.
    fn write_vi63le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`write_vu63be`][Writable::write_vu63be] but for signed integers.
    fn write_vi63be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`write_vu63ler`][Writable::write_vu63ler] but for signed integers.
    fn write_vi63ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`write_vu63ber`][Writable::write_vu63ber] but for signed integers.
    fn write_vi63ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(8, num)
    }

    /// Writes a signed 127-bit variable-length integer in little-endian byte order at the current position.
    ///
    /// This works like [`write_vu127le`][Writable::write_vu127le] but for signed integers.
    fn write_vi127le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in big-endian byte order at the current position.
    ///
    /// This works like [`write_vu127be`][Writable::write_vu127be] but for signed integers.
    fn write_vi127be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// This works like [`write_vu127ler`][Writable::write_vu127ler] but for signed integers.
    fn write_vi127ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// This works like [`write_vu127ber`][Writable::write_vu127ber] but for signed integers.
    fn write_vi127ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(16, num)
    }

    /// Writes a signed integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(3);
    ///
    /// writer.write_ixle(3, -0x01_02_03).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0xfd, 0xfd, 0xfe]);
    /// ```
    fn write_ixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxle(size, signed_to_unsigned(num, size))
    }

    /// Writes a signed integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(3);
    ///
    /// writer.write_ixbe(3, -0x01_02_03).unwrap();
    ///
    /// let data = dh::data::close(writer);
    /// assert_eq!(data, vec![0xfe, 0xfd, 0xfd]);
    /// ```
    fn write_ixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxbe(size, signed_to_unsigned(num, size))
    }

    /// Writes a signed variable-length integer in little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vixle(3, -0x01_02_03).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vixle(3).unwrap(), -0x01_02_03);
    /// ```
    fn write_vixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxle(size, signed_to_unsigned_vi(num, size))
    }

    /// Writes a signed variable-length integer in big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vixbe(3, -0x01_02_03).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vixbe(3).unwrap(), -0x01_02_03);
    /// ```
    fn write_vixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxbe(size, signed_to_unsigned_vi(num, size))
    }

    /// Writes a signed variable-length integer in reversed little-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vixler(3, -0x01_02_03).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vixler(3).unwrap(), -0x01_02_03);
    /// ```
    fn write_vixler(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxler(size, signed_to_unsigned_vi(num, size))
    }

    /// Writes a signed variable-length integer in reversed big-endian byte order at the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut rw = dh::data::rw_empty();
    ///
    /// rw.write_vixber(3, -0x01_02_03).unwrap();
    ///
    /// rw.rewind();
    /// assert_eq!(rw.read_vixber(3).unwrap(), -0x01_02_03);
    /// ```
    fn write_vixber(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxber(size, signed_to_unsigned_vi(num, size))
    }
}
