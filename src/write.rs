use std::io::{Result, Seek, Write};

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

pub trait Writable
where
    Self: Write + Seek,
{
    fn alloc(&mut self, len: u64) -> Result<()>;
    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;
    fn close(self) -> Result<()>;

    fn rewind(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(0))
    }

    fn end(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::End(0))
    }

    fn to(&mut self, pos: u64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(pos))
    }

    fn jump(&mut self, pos: i64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Current(pos))
    }

    fn size(&mut self) -> Result<u64> {
        let pos_before = self.stream_position()?;
        let size = self.end();
        self.to(pos_before)?;
        size
    }

    fn write_utf8_at(&mut self, pos: u64, s: &String) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_utf8(s)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u8_at(&mut self, pos: u64, num: u8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u16le_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u16be_at(&mut self, pos: u64, num: u16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u32le_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u32be_at(&mut self, pos: u64, num: u32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u64le_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u64be_at(&mut self, pos: u64, num: u64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u128le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_u128be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_u128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu7_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu7r_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu7r(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu15le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu15be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu15ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu15ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu15ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu31le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu31be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu31ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu31ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu31ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu63le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu63be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu63ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu63ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu63ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu127le_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu127be_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu127ler_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ler(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vu127ber_at(&mut self, pos: u64, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vu127ber(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_uxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_uxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_uxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vuxle_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vuxler_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vuxbe_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vuxber_at(&mut self, pos: u64, size: u8, num: u128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vuxber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i8_at(&mut self, pos: u64, num: i8) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i8(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i16le_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i16be_at(&mut self, pos: u64, num: i16) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i16be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i32le_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i32be_at(&mut self, pos: u64, num: i32) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i32be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i64le_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i64be_at(&mut self, pos: u64, num: i64) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i64be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i128le_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128le(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_i128be_at(&mut self, pos: u64, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_i128be(num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_ixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_ixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_ixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vixle_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixle(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vixler_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixler(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vixbe_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixbe(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_vixber_at(&mut self, pos: u64, size: u8, num: i128) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_vixber(size, num)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_utf8(&mut self, s: &String) -> Result<()> {
        self.write_all(s.as_bytes())
    }

    fn write_u8(&mut self, num: u8) -> Result<()> {
        self.write_all(&[num])
    }

    fn write_u16le(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_u16be(&mut self, num: u16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_u32le(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_u32be(&mut self, num: u32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_u64le(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_u64be(&mut self, num: u64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_u128le(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_u128be(&mut self, num: u128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_vu7(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(1, num)
    }

    fn write_vu7r(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(1, num)
    }

    fn write_vu15le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(2, num)
    }

    fn write_vu15be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(2, num)
    }

    fn write_vu15ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(2, num)
    }

    fn write_vu15ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(2, num)
    }

    fn write_vu31le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(4, num)
    }

    fn write_vu31be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(4, num)
    }

    fn write_vu31ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(4, num)
    }

    fn write_vu31ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(4, num)
    }

    fn write_vu63le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(8, num)
    }

    fn write_vu63be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(8, num)
    }

    fn write_vu63ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(8, num)
    }

    fn write_vu63ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(8, num)
    }

    fn write_vu127le(&mut self, num: u128) -> Result<()> {
        self.write_vuxle(16, num)
    }

    fn write_vu127be(&mut self, num: u128) -> Result<()> {
        self.write_vuxbe(16, num)
    }

    fn write_vu127ler(&mut self, num: u128) -> Result<()> {
        self.write_vuxler(16, num)
    }

    fn write_vu127ber(&mut self, num: u128) -> Result<()> {
        self.write_vuxber(16, num)
    }

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

    fn write_vuxle(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, false);
        self.write_all(&buf)
    }

    fn write_vuxler(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, false, true);
        self.write_all(&buf)
    }

    fn write_vuxbe(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, false);
        self.write_all(&buf)
    }

    fn write_vuxber(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true, true);
        self.write_all(&buf)
    }

    fn write_i8(&mut self, num: i8) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_i16le(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_i16be(&mut self, num: i16) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_i32le(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_i32be(&mut self, num: i32) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_i64le(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_i64be(&mut self, num: i64) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_i128le(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_le_bytes())
    }

    fn write_i128be(&mut self, num: i128) -> Result<()> {
        self.write_all(&num.to_be_bytes())
    }

    fn write_ixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxle(size, signed_to_unsigned(num))
    }

    fn write_ixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_uxbe(size, signed_to_unsigned(num))
    }

    fn write_vixle(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxle(size, signed_to_unsigned(num))
    }

    fn write_vixler(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxler(size, signed_to_unsigned(num))
    }

    fn write_vixbe(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxbe(size, signed_to_unsigned(num))
    }

    fn write_vixber(&mut self, size: u8, num: i128) -> Result<()> {
        self.write_vuxber(size, signed_to_unsigned(num))
    }
}
