use std::io::{Result, Seek, Write};

fn serialize_vuxle(size: u8, num: u128, be: bool) -> Vec<u8> {
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
    println!("{:?}", buf);
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
        let buf = serialize_vuxle(size, num, false);
        self.write_all(&buf)
    }

    fn write_vuxbe(&mut self, size: u8, num: u128) -> Result<()> {
        let buf = serialize_vuxle(size, num, true);
        self.write_all(&buf)
    }

    fn write_vu7(&mut self, num: u128) -> Result<()> {
        let mut num = num;
        let mut buf = Vec::new();
        while num >= 0x80 {
            buf.push((num & 0x7F) as u8 | 0x80);
            num >>= 7;
        }
        buf.push(num as u8);
        self.write_all(&buf)
    }

    fn write_vu15le(&mut self, num: u128) -> Result<()> {
        let buf = serialize_vuxle(2, num, false);
        self.write_all(&buf)
    }

    fn write_vu31le(&mut self, num: u128) -> Result<()> {
        let buf = serialize_vuxle(4, num, false);
        self.write_all(&buf)
    }
}
