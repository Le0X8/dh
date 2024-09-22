use std::{
    io::{ErrorKind, Read, Result, Seek},
    vec,
};

pub trait Readable
where
    Self: Read + Seek,
{
    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;
    fn close(self) -> Result<()>;

    fn rewind(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(0))
    }

    fn to(&mut self, pos: u64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(pos))
    }

    fn jump(&mut self, pos: i64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Current(pos))
    }

    fn size(&mut self) -> Result<u64> {
        let pos_before = match self.stream_position() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        let size = self.seek(std::io::SeekFrom::End(0));
        match self.seek(std::io::SeekFrom::Start(pos_before)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        size
    }

    fn read_utf8_at(&mut self, pos: u64, len: u64) -> Result<String> {
        let pos_before = match self.stream_position() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        match self.seek(std::io::SeekFrom::Start(pos)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        let s = self.read_utf8(len);
        match self.seek(std::io::SeekFrom::Start(pos_before)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        s
    }

    fn read_utf8(&mut self, length: u64) -> Result<String> {
        let length = length as usize;
        let mut buf = vec![0; length];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != length {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(match String::from_utf8(buf) {
                    Ok(str) => str,
                    Err(_) => return Err(ErrorKind::InvalidData.into()),
                })
            }
            Err(err) => Err(err),
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        match self.read_uxle(1) {
            Ok(byte) => Ok(byte as u8),
            Err(err) => Err(err),
        }
    }

    fn read_u16le(&mut self) -> Result<u16> {
        match self.read_uxle(2) {
            Ok(byte) => Ok(byte as u16),
            Err(err) => Err(err),
        }
    }

    fn read_u16be(&mut self) -> Result<u16> {
        match self.read_uxbe(2) {
            Ok(byte) => Ok(byte as u16),
            Err(err) => Err(err),
        }
    }

    fn read_u32le(&mut self) -> Result<u32> {
        match self.read_uxle(4) {
            Ok(byte) => Ok(byte as u32),
            Err(err) => Err(err),
        }
    }

    fn read_u32be(&mut self) -> Result<u32> {
        match self.read_uxbe(4) {
            Ok(byte) => Ok(byte as u32),
            Err(err) => Err(err),
        }
    }

    fn read_u64le(&mut self) -> Result<u64> {
        match self.read_uxle(8) {
            Ok(byte) => Ok(byte as u64),
            Err(err) => Err(err),
        }
    }

    fn read_u64be(&mut self) -> Result<u64> {
        match self.read_uxbe(8) {
            Ok(byte) => Ok(byte as u64),
            Err(err) => Err(err),
        }
    }

    fn read_u128le(&mut self) -> Result<u128> {
        self.read_uxle(16)
    }

    fn read_u128be(&mut self) -> Result<u128> {
        self.read_uxbe(16)
    }

    fn read_vu7(&mut self) -> Result<u128> {
        self.read_vuxle(1)
    }

    fn read_vu7r(&mut self) -> Result<u128> {
        self.read_vuxler(1)
    }

    fn read_vu15le(&mut self) -> Result<u128> {
        self.read_vuxle(2)
    }

    fn read_vu15be(&mut self) -> Result<u128> {
        self.read_vuxbe(2)
    }

    fn read_vu15ler(&mut self) -> Result<u128> {
        self.read_vuxler(2)
    }

    fn read_vu15ber(&mut self) -> Result<u128> {
        self.read_vuxber(2)
    }

    fn read_vu31le(&mut self) -> Result<u128> {
        self.read_vuxle(4)
    }

    fn read_vu31be(&mut self) -> Result<u128> {
        self.read_vuxbe(4)
    }

    fn read_vu31ler(&mut self) -> Result<u128> {
        self.read_vuxler(4)
    }

    fn read_vu31ber(&mut self) -> Result<u128> {
        self.read_vuxber(4)
    }

    fn read_vu63le(&mut self) -> Result<u128> {
        self.read_vuxle(8)
    }

    fn read_vu63be(&mut self) -> Result<u128> {
        self.read_vuxbe(8)
    }

    fn read_vu63ler(&mut self) -> Result<u128> {
        self.read_vuxler(8)
    }

    fn read_vu63ber(&mut self) -> Result<u128> {
        self.read_vuxber(8)
    }

    fn read_vu127le(&mut self) -> Result<u128> {
        self.read_vuxle(16)
    }

    fn read_vu127be(&mut self) -> Result<u128> {
        self.read_vuxbe(16)
    }

    fn read_vu127ler(&mut self) -> Result<u128> {
        self.read_vuxler(16)
    }

    fn read_vu127ber(&mut self) -> Result<u128> {
        self.read_vuxber(16)
    }

    fn read_uxle(&mut self, size: u8) -> Result<u128> {
        let mut buffer = [0u8; 16];
        match self.read_exact(&mut buffer[0..size as usize]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << (i * 8);
        }

        Ok(result)
    }

    fn read_uxbe(&mut self, size: u8) -> Result<u128> {
        let mut buffer = [0u8; 16];
        match self.read_exact(&mut buffer[0..size as usize]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }

    fn read_vuxle(&mut self, size: u8) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        loop {
            let bytes = self.read_uxle(size)?;
            if bytes & (1 << (size * 8 - 1)) == 0 {
                result |= (bytes & !(1 << (size * 8 - 1))) << shift;
                break;
            }
            result |= (bytes & !(1 << (size * 8 - 1))) << shift;
            shift += size * 8 - 1;
        }
        Ok(result)
    }

    fn read_vuxbe(&mut self, size: u8) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        loop {
            let bytes = self.read_uxbe(size)?;
            if bytes & (1 << (size * 8 - 1)) == 0 {
                result |= (bytes & !(1 << (size * 8 - 1))) << shift;
                break;
            }
            result |= (bytes & !(1 << (size * 8 - 1))) << shift;
            shift += size * 8 - 1;
        }
        Ok(result)
    }

    fn read_vuxler(&mut self, size: u8) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        let mut bytes = [0u128; 16];
        let mut i = 0;
        loop {
            let b = self.read_uxle(size)?;
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
        Ok(result)
    }

    fn read_vuxber(&mut self, size: u8) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        let mut bytes = [0u128; 16];
        let mut i = 0;
        loop {
            let b = self.read_uxbe(size)?;
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
        Ok(result)
    }

    fn read_i8(&mut self) -> Result<i8> {
        match self.read_ixle(1) {
            Ok(byte) => Ok(byte as i8),
            Err(err) => Err(err),
        }
    }

    fn read_i16le(&mut self) -> Result<i16> {
        match self.read_ixle(2) {
            Ok(byte) => Ok(byte as i16),
            Err(err) => Err(err),
        }
    }

    fn read_i16be(&mut self) -> Result<i16> {
        match self.read_ixbe(2) {
            Ok(byte) => Ok(byte as i16),
            Err(err) => Err(err),
        }
    }

    fn read_i32le(&mut self) -> Result<i32> {
        match self.read_ixle(4) {
            Ok(byte) => Ok(byte as i32),
            Err(err) => Err(err),
        }
    }

    fn read_i32be(&mut self) -> Result<i32> {
        match self.read_ixbe(4) {
            Ok(byte) => Ok(byte as i32),
            Err(err) => Err(err),
        }
    }

    fn read_i64le(&mut self) -> Result<i64> {
        match self.read_ixle(8) {
            Ok(byte) => Ok(byte as i64),
            Err(err) => Err(err),
        }
    }

    fn read_i64be(&mut self) -> Result<i64> {
        match self.read_ixbe(8) {
            Ok(byte) => Ok(byte as i64),
            Err(err) => Err(err),
        }
    }

    fn read_i128le(&mut self) -> Result<i128> {
        self.read_ixle(16)
    }

    fn read_i128be(&mut self) -> Result<i128> {
        self.read_ixbe(16)
    }

    fn read_ixle(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        match self.read_exact(&mut buffer[0..size as usize]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << (i * 8);
        }

        Ok(result)
    }

    fn read_ixbe(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        match self.read_exact(&mut buffer[0..size as usize]) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }
}
