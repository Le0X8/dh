use std::io::{Result, Write};

use crate::{DataType, Seekable};

fn signed_to_unsigned(num: i128) -> u128 {
    u128::from_ne_bytes(num.to_ne_bytes()) // there might be a better way to do this but this worked instantly so I leave it this way
}

fn serialize_vuxle(size: u8, num: u128, be: bool, rev: bool) -> Vec<u8> {
    let mut num = num;
    let mut buf = Vec::new();
    let shift = (8 * size) - 1;
    let max_size = 1 << shift;
    let mask = max_size - 1;
    while num >= max_size {
        buf.push((num & mask) | max_size);
        num >>= shift;
    }
    buf.push(num);
    if rev {
        buf.reverse();
    }
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
pub trait Writable<'a>
where
    Self: Write + Seekable,
{
    /// Pre-allocates space in the data stream.
    fn alloc(&mut self, len: u64) -> Result<()>;

    /// Locks the source exclusively for the current process.
    fn lock(&mut self, block: bool) -> Result<()>;

    /// Unlocks the source for other processes.
    fn unlock(&mut self) -> Result<()>;

    /// Closes the writer and can return the target if it was moved or references it.
    fn close(self) -> Result<Option<DataType<'a>>>;

    /// Writes an UTF-8-encoded string at a specific position.
    fn write_utf8_at(&mut self, pos: u64, s: &String) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_utf8(s)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 8-bit integer at a specific position.
    fn write_u8_at(&mut self, pos: u64, num: u8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 16-bit integer in little-endian byte order at a specific position.
    fn write_u16le_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 16-bit integer in big-endian byte order at a specific position.
    fn write_u16be_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 32-bit integer in little-endian byte order at a specific position.
    fn write_u32le_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 32-bit integer in big-endian byte order at a specific position.
    fn write_u32be_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 64-bit integer in little-endian byte order at a specific position.
    fn write_u64le_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 64-bit integer in big-endian byte order at a specific position.
    fn write_u64be_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 128-bit integer in little-endian byte order at a specific position.
    fn write_u128le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 128-bit integer in big-endian byte order at a specific position.
    fn write_u128be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 7-bit variable-length integer at a specific position.
    fn write_vu7_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 7-bit variable-length integer in reversed byte order at a specific position.
    fn write_vu7r_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7r(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vu15le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vu15be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vu15ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vu15ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vu31le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vu31be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vu31ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vu31ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vu63le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vu63be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vu63ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vu63ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vu127le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vu127be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vu127ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vu127ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned integer in little-endian byte order at a specific position.
    fn write_uxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned integer in big-endian byte order at a specific position.
    fn write_uxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in little-endian byte order at a specific position.
    fn write_vuxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vuxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vuxler_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an unsigned variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vuxber_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 8-bit integer at a specific position.
    fn write_i8_at(&mut self, pos: u64, num: i8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 16-bit integer in little-endian byte order at a specific position.
    fn write_i16le_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 16-bit integer in big-endian byte order at a specific position.
    fn write_i16be_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 32-bit integer in little-endian byte order at a specific position.
    fn write_i32le_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 32-bit integer in big-endian byte order at a specific position.
    fn write_i32be_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 64-bit integer in little-endian byte order at a specific position.
    fn write_i64le_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 64-bit integer in big-endian byte order at a specific position.
    fn write_i64be_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 128-bit integer in little-endian byte order at a specific position.
    fn write_i128le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 128-bit integer in big-endian byte order at a specific position.
    fn write_i128be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 7-bit variable-length integer at a specific position.
    fn write_vi7_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi7(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 7-bit variable-length integer in reversed byte order at a specific position.
    fn write_vi7r_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi7r(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vi15le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vi15be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vi15ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 15-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vi15ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi15ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vi31le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vi31be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vi31ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 31-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vi31ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi31ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vi63le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vi63be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 63-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vi63ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi63ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in little-endian byte order at a specific position.
    fn write_vi127le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in big-endian byte order at a specific position.
    fn write_vi127be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vi127ler_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed 127-bit variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vi127ber_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vi127ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed integer in little-endian byte order at a specific position.
    fn write_ixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed integer in big-endian byte order at a specific position.
    fn write_ixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in little-endian byte order at a specific position.
    fn write_vixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in reversed little-endian byte order at a specific position.
    fn write_vixler_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in big-endian byte order at a specific position.
    fn write_vixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes a signed variable-length integer in reversed big-endian byte order at a specific position.
    fn write_vixber_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    /// Writes an UTF-8-encoded string at the current position.
    fn write_utf8(&mut self, s: &String) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    /// Writes an unsigned 8-bit integer at the current position.
    fn write_u8(&mut self, num: u8) -> Result<()> {
        self.write_all(&[num])
    }

    /// Writes an unsigned 16-bit integer in little-endian byte order at the current position.
    fn write_u16le(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 16-bit integer in big-endian byte order at the current position.
    fn write_u16be(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 32-bit integer in little-endian byte order at the current position.
    fn write_u32le(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 32-bit integer in big-endian byte order at the current position.
    fn write_u32be(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 64-bit integer in little-endian byte order at the current position.
    fn write_u64le(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 64-bit integer in big-endian byte order at the current position.
    fn write_u64be(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 128-bit integer in little-endian byte order at the current position.
    fn write_u128le(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes an unsigned 128-bit integer in big-endian byte order at the current position.
    fn write_u128be(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes an unsigned 7-bit variable-length integer at the current position.
    fn write_vu7(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(1, num)
    }

    /// Writes an unsigned 7-bit variable-length integer in reversed byte order at the current position.
    fn write_vu7r(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(1, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in little-endian byte order at the current position.
    fn write_vu15le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in big-endian byte order at the current position.
    fn write_vu15be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vu15ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(2, num)
    }

    /// Writes an unsigned 15-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vu15ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(2, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in little-endian byte order at the current position.
    fn write_vu31le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in big-endian byte order at the current position.
    fn write_vu31be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vu31ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(4, num)
    }

    /// Writes an unsigned 31-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vu31ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(4, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in little-endian byte order at the current position.
    fn write_vu63le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in big-endian byte order at the current position.
    fn write_vu63be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vu63ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(8, num)
    }

    /// Writes an unsigned 63-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vu63ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(8, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in little-endian byte order at the current position.
    fn write_vu127le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in big-endian byte order at the current position.
    fn write_vu127be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vu127ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(16, num)
    }

    /// Writes an unsigned 127-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vu127ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(16, num)
    }

    /// Writes an unsigned integer in little-endian byte order at the current position.
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
    fn write_vuxle(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, false);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in big-endian byte order at the current position.
    fn write_vuxler(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, true);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in big-endian byte order at the current position.
    fn write_vuxbe(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, false);
        self.write_all(&buf)
    }

    /// Writes an unsigned variable-length integer in big-endian byte order at the current position.
    fn write_vuxber(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, true);
        self.write_all(&buf)
    }

    /// Writes a signed 8-bit integer at the current position.
    fn write_i8(&mut self, num: i8) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 16-bit integer in little-endian byte order at the current position.
    fn write_i16le(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 16-bit integer in big-endian byte order at the current position.
    fn write_i16be(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 32-bit integer in little-endian byte order at the current position.
    fn write_i32le(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 32-bit integer in big-endian byte order at the current position.
    fn write_i32be(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 64-bit integer in little-endian byte order at the current position.
    fn write_i64le(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 64-bit integer in big-endian byte order at the current position.
    fn write_i64be(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 128-bit integer in little-endian byte order at the current position.
    fn write_i128le(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    /// Writes a signed 128-bit integer in big-endian byte order at the current position.
    fn write_i128be(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    /// Writes a signed 7-bit variable-length integer at the current position.
    fn write_vi7(&mut self, num: i128) -> Result<()> {
        self.write_vixle(1, num)
    }

    /// Writes a signed 7-bit variable-length integer in reversed byte order at the current position.
    fn write_vi7r(&mut self, num: i128) -> Result<()> {
        self.write_vixler(1, num)
    }

    /// Writes a signed 15-bit variable-length integer in little-endian byte order at the current position.
    fn write_vi15le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in big-endian byte order at the current position.
    fn write_vi15be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vi15ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(2, num)
    }

    /// Writes a signed 15-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vi15ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(2, num)
    }

    /// Writes a signed 31-bit variable-length integer in little-endian byte order at the current position.
    fn write_vi31le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in big-endian byte order at the current position.
    fn write_vi31be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vi31ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(4, num)
    }

    /// Writes a signed 31-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vi31ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(4, num)
    }

    /// Writes a signed 63-bit variable-length integer in little-endian byte order at the current position.
    fn write_vi63le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in big-endian byte order at the current position.
    fn write_vi63be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vi63ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(8, num)
    }

    /// Writes a signed 63-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vi63ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(8, num)
    }

    /// Writes a signed 127-bit variable-length integer in little-endian byte order at the current position.
    fn write_vi127le(&mut self, num: i128) -> Result<()> {
        self.write_vixle(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in big-endian byte order at the current position.
    fn write_vi127be(&mut self, num: i128) -> Result<()> {
        self.write_vixbe(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in reversed little-endian byte order at the current position.
    fn write_vi127ler(&mut self, num: i128) -> Result<()> {
        self.write_vixler(16, num)
    }

    /// Writes a signed 127-bit variable-length integer in reversed big-endian byte order at the current position.
    fn write_vi127ber(&mut self, num: i128) -> Result<()> {
        self.write_vixber(16, num)
    }

    /// Writes a signed integer in little-endian byte order at the current position.
    fn write_ixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxle(size, signed_to_unsigned(num))
    }

    /// Writes a signed integer in big-endian byte order at the current position.
    fn write_ixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxbe(size, signed_to_unsigned(num))
    }

    /// Writes a signed variable-length integer in little-endian byte order at the current position.
    fn write_vixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxle(size, signed_to_unsigned(num))
    }

    /// Writes a signed variable-length integer in reversed little-endian byte order at the current position.
    fn write_vixler(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxler(size, signed_to_unsigned(num))
    }

    /// Writes a signed variable-length integer in big-endian byte order at the current position.
    fn write_vixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxbe(size, signed_to_unsigned(num))
    }

    /// Writes a signed variable-length integer in reversed big-endian byte order at the current position.
    fn write_vixber(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxber(size, signed_to_unsigned(num))
    }
}
