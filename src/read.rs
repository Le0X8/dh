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
        let mut buf = [0; 1];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 1 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u8::from_ne_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u16le(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 2 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u16::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u16be(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 2 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u16::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u32le(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 4 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u32::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u32be(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 4 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u32::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u64le(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 8 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u64::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u64be(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 8 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u64::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u128le(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 16 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u128::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_u128be(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 16 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(u128::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_vu7(&mut self) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        loop {
            let byte = self.read_u8()?;
            if byte & 0x80 == 0 {
                result |= ((byte as u128) & 0x7f) << shift;
                break;
            }
            result |= ((byte as u128) & 0x7f) << shift;
            shift += 7;
        }
        Ok(result)
    }

    fn read_vu7r(&mut self) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        let mut bytes = [0u8; 16];
        let mut i = 0;
        loop {
            let byte = self.read_u8()?;
            bytes[i] = byte;
            i += 1;
            if byte & 0x80 == 0 {
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
            result |= ((*byte as u128) & 0x7f) << shift;
            shift += 7;
        }
        Ok(result)
    }

    fn read_vu15le(&mut self) -> Result<u128> {
        let mut result = 0;
        let mut shift = 0u8;
        loop {
            let bytes = self.read_u16le()?;
            if bytes & 0x8000 == 0 {
                println!("bytex: {:x}", bytes);
                result |= ((bytes as u128) & 0x7fff) << shift;
                break;
            }
            println!("bytes: {:x}", bytes);
            result |= ((bytes as u128) & 0x7fff) << shift;
            shift += 15;
        }
        Ok(result)
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

    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 1 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i8::from_ne_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i16le(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 2 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i16::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i16be(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 2 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i16::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i32le(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 4 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i32::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i32be(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 4 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i32::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i64le(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 8 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i64::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i64be(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 8 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i64::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i128le(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 16 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i128::from_le_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }

    fn read_i128be(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        match self.read(&mut buf) {
            Ok(read_length) => {
                if read_length != 16 {
                    return Err(ErrorKind::UnexpectedEof.into());
                }

                Ok(i128::from_be_bytes(buf))
            }
            Err(err) => Err(err),
        }
    }
}
