use std::{
    io::{ErrorKind, Read, Result, Seek},
    vec,
};

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

    fn read_utf8_at(&mut self, pos: u64, len: u64) -> Result<String> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_utf8(len);
        self.to(pos_before)?;
        result
    }

    fn read_u8_at(&mut self, pos: u64) -> Result<u8> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u8();
        self.to(pos_before)?;
        result
    }

    fn read_u16le_at(&mut self, pos: u64) -> Result<u16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u16le();
        self.to(pos_before)?;
        result
    }

    fn read_u16be_at(&mut self, pos: u64) -> Result<u16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u16be();
        self.to(pos_before)?;
        result
    }

    fn read_u32le_at(&mut self, pos: u64) -> Result<u32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u32le();
        self.to(pos_before)?;
        result
    }

    fn read_u32be_at(&mut self, pos: u64) -> Result<u32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u32be();
        self.to(pos_before)?;
        result
    }

    fn read_u64le_at(&mut self, pos: u64) -> Result<u64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u64le();
        self.to(pos_before)?;
        result
    }

    fn read_u64be_at(&mut self, pos: u64) -> Result<u64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_u64be();
        self.to(pos_before)?;
        result
    }

    fn read_vu7_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu7();
        self.to(pos_before)?;
        result
    }

    fn read_vu7r_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu7r();
        self.to(pos_before)?;
        result
    }

    fn read_vu15le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15le();
        self.to(pos_before)?;
        result
    }

    fn read_vu15be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15be();
        self.to(pos_before)?;
        result
    }

    fn read_vu15ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15ler();
        self.to(pos_before)?;
        result
    }

    fn read_vu15ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu15ber();
        self.to(pos_before)?;
        result
    }

    fn read_vu31le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31le();
        self.to(pos_before)?;
        result
    }

    fn read_vu31be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31be();
        self.to(pos_before)?;
        result
    }

    fn read_vu31ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31ler();
        self.to(pos_before)?;
        result
    }

    fn read_vu31ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu31ber();
        self.to(pos_before)?;
        result
    }

    fn read_vu63le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63le();
        self.to(pos_before)?;
        result
    }

    fn read_vu63be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63be();
        self.to(pos_before)?;
        result
    }

    fn read_vu63ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63ler();
        self.to(pos_before)?;
        result
    }

    fn read_vu63ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu63ber();
        self.to(pos_before)?;
        result
    }

    fn read_vu127le_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127le();
        self.to(pos_before)?;
        result
    }

    fn read_vu127be_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127be();
        self.to(pos_before)?;
        result
    }

    fn read_vu127ler_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127ler();
        self.to(pos_before)?;
        result
    }

    fn read_vu127ber_at(&mut self, pos: u64) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vu127ber();
        self.to(pos_before)?;
        result
    }

    fn read_uxle_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_uxle(size);
        self.to(pos_before)?;
        result
    }

    fn read_uxbe_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_uxbe(size);
        self.to(pos_before)?;
        result
    }

    fn read_vuxle_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxle(size);
        self.to(pos_before)?;
        result
    }

    fn read_vuxbe_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxbe(size);
        self.to(pos_before)?;
        result
    }

    fn read_vuxler_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxler(size);
        self.to(pos_before)?;
        result
    }

    fn read_vuxber_at(&mut self, pos: u64, size: u8) -> Result<u128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vuxber(size);
        self.to(pos_before)?;
        result
    }

    fn read_i8_at(&mut self, pos: u64) -> Result<i8> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i8();
        self.to(pos_before)?;
        result
    }

    fn read_i16le_at(&mut self, pos: u64) -> Result<i16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i16le();
        self.to(pos_before)?;
        result
    }

    fn read_i16be_at(&mut self, pos: u64) -> Result<i16> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i16be();
        self.to(pos_before)?;
        result
    }

    fn read_i32le_at(&mut self, pos: u64) -> Result<i32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i32le();
        self.to(pos_before)?;
        result
    }

    fn read_i32be_at(&mut self, pos: u64) -> Result<i32> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i32be();
        self.to(pos_before)?;
        result
    }

    fn read_i64le_at(&mut self, pos: u64) -> Result<i64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i64le();
        self.to(pos_before)?;
        result
    }

    fn read_i64be_at(&mut self, pos: u64) -> Result<i64> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i64be();
        self.to(pos_before)?;
        result
    }

    fn read_i128le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i128le();
        self.to(pos_before)?;
        result
    }

    fn read_i128be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_i128be();
        self.to(pos_before)?;
        result
    }

    fn read_vi7_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi7();
        self.to(pos_before)?;
        result
    }

    fn read_vi7r_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi7r();
        self.to(pos_before)?;
        result
    }

    fn read_vi15le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15le();
        self.to(pos_before)?;
        result
    }

    fn read_vi15be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15be();
        self.to(pos_before)?;
        result
    }

    fn read_vi15ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15ler();
        self.to(pos_before)?;
        result
    }

    fn read_vi15ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi15ber();
        self.to(pos_before)?;
        result
    }

    fn read_vi31le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31le();
        self.to(pos_before)?;
        result
    }

    fn read_vi31be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31be();
        self.to(pos_before)?;
        result
    }

    fn read_vi31ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31ler();
        self.to(pos_before)?;
        result
    }

    fn read_vi31ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi31ber();
        self.to(pos_before)?;
        result
    }

    fn read_vi63le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63le();
        self.to(pos_before)?;
        result
    }

    fn read_vi63be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63be();
        self.to(pos_before)?;
        result
    }

    fn read_vi63ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63ler();
        self.to(pos_before)?;
        result
    }

    fn read_vi63ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi63ber();
        self.to(pos_before)?;
        result
    }

    fn read_vi127le_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127le();
        self.to(pos_before)?;
        result
    }

    fn read_vi127be_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127be();
        self.to(pos_before)?;
        result
    }

    fn read_vi127ler_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127ler();
        self.to(pos_before)?;
        result
    }

    fn read_vi127ber_at(&mut self, pos: u64) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vi127ber();
        self.to(pos_before)?;
        result
    }

    fn read_ixle_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_ixle(size);
        self.to(pos_before)?;
        result
    }

    fn read_ixbe_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_ixbe(size);
        self.to(pos_before)?;
        result
    }

    fn read_vixle_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixle(size);
        self.to(pos_before)?;
        result
    }

    fn read_vixbe_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixbe(size);
        self.to(pos_before)?;
        result
    }

    fn read_vixler_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixler(size);
        self.to(pos_before)?;
        result
    }

    fn read_vixber_at(&mut self, pos: u64, size: u8) -> Result<i128> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        let result = self.read_vixber(size);
        self.to(pos_before)?;
        result
    }

    fn read_utf8(&mut self, length: u64) -> Result<String> {
        let mut buf = vec![0; length as usize];
        self.read_exact(&mut buf)?;
        Ok(match String::from_utf8(buf) {
            Ok(str) => str,
            Err(_) => return Err(ErrorKind::InvalidData.into()),
        })
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_uxle(1)? as u8)
    }

    fn read_u16le(&mut self) -> Result<u16> {
        Ok(self.read_uxle(2)? as u16)
    }

    fn read_u16be(&mut self) -> Result<u16> {
        Ok(self.read_uxbe(2)? as u16)
    }

    fn read_u32le(&mut self) -> Result<u32> {
        Ok(self.read_uxle(4)? as u32)
    }

    fn read_u32be(&mut self) -> Result<u32> {
        Ok(self.read_uxbe(4)? as u32)
    }

    fn read_u64le(&mut self) -> Result<u64> {
        Ok(self.read_uxle(8)? as u64)
    }

    fn read_u64be(&mut self) -> Result<u64> {
        Ok(self.read_uxbe(8)? as u64)
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
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << (i * 8);
        }

        Ok(result)
    }

    fn read_uxbe(&mut self, size: u8) -> Result<u128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0u128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as u128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }

    fn read_vuxle(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vux(&mut |s| self.read_uxle(s), size)?.0)
    }

    fn read_vuxbe(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vux(&mut |s| self.read_uxbe(s), size)?.0)
    }

    fn read_vuxler(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vuxr(&mut |s| self.read_uxle(s), size)?.0)
    }

    fn read_vuxber(&mut self, size: u8) -> Result<u128> {
        Ok(parse_vuxr(&mut |s| self.read_uxbe(s), size)?.0)
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_ixle(1)? as i8)
    }

    fn read_i16le(&mut self) -> Result<i16> {
        Ok(self.read_ixle(2)? as i16)
    }

    fn read_i16be(&mut self) -> Result<i16> {
        Ok(self.read_ixbe(2)? as i16)
    }

    fn read_i32le(&mut self) -> Result<i32> {
        Ok(self.read_ixle(4)? as i32)
    }

    fn read_i32be(&mut self) -> Result<i32> {
        Ok(self.read_ixbe(4)? as i32)
    }

    fn read_i64le(&mut self) -> Result<i64> {
        Ok(self.read_ixle(8)? as i64)
    }

    fn read_i64be(&mut self) -> Result<i64> {
        Ok(self.read_ixbe(8)? as i64)
    }

    fn read_i128le(&mut self) -> Result<i128> {
        self.read_ixle(16)
    }

    fn read_i128be(&mut self) -> Result<i128> {
        self.read_ixbe(16)
    }

    fn read_vi7(&mut self) -> Result<i128> {
        self.read_vixle(1)
    }

    fn read_vi7r(&mut self) -> Result<i128> {
        self.read_vixler(1)
    }

    fn read_vi15le(&mut self) -> Result<i128> {
        self.read_vixle(2)
    }

    fn read_vi15be(&mut self) -> Result<i128> {
        self.read_vixbe(2)
    }

    fn read_vi15ler(&mut self) -> Result<i128> {
        self.read_vixler(2)
    }

    fn read_vi15ber(&mut self) -> Result<i128> {
        self.read_vixber(2)
    }

    fn read_vi31le(&mut self) -> Result<i128> {
        self.read_vixle(4)
    }

    fn read_vi31be(&mut self) -> Result<i128> {
        self.read_vixbe(4)
    }

    fn read_vi31ler(&mut self) -> Result<i128> {
        self.read_vixler(4)
    }

    fn read_vi31ber(&mut self) -> Result<i128> {
        self.read_vixber(4)
    }

    fn read_vi63le(&mut self) -> Result<i128> {
        self.read_vixle(8)
    }

    fn read_vi63be(&mut self) -> Result<i128> {
        self.read_vixbe(8)
    }

    fn read_vi63ler(&mut self) -> Result<i128> {
        self.read_vixler(8)
    }

    fn read_vi63ber(&mut self) -> Result<i128> {
        self.read_vixber(8)
    }

    fn read_vi127le(&mut self) -> Result<i128> {
        self.read_vixle(16)
    }

    fn read_vi127be(&mut self) -> Result<i128> {
        self.read_vixbe(16)
    }

    fn read_vi127ler(&mut self) -> Result<i128> {
        self.read_vixler(16)
    }

    fn read_vi127ber(&mut self) -> Result<i128> {
        self.read_vixber(16)
    }

    fn read_ixle(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << (i * 8);
        }

        Ok(result)
    }

    fn read_ixbe(&mut self, size: u8) -> Result<i128> {
        let mut buffer = [0u8; 16];
        self.read_exact(&mut buffer[0..size as usize])?;

        let mut result = 0i128;
        for (i, &byte) in buffer.iter().enumerate().take(size as usize) {
            result |= (byte as i128) << ((size - (i as u8) - 1) * 8);
        }

        Ok(result)
    }

    fn read_vixle(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxle(s);
        let result = parse_vux(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        let block_len = size * 8 - 1;
        let bit_len = block_len * length;
        let negative = unsigned & (1 << (bit_len - 1)) != 0;

        let int = (unsigned & !(1 << (bit_len - 1))) as i128;

        if negative {
            Ok(-int)
        } else {
            Ok(int)
        }
    }

    fn read_vixbe(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxbe(s);
        let result = parse_vux(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        let block_len = size * 8 - 1;
        let bit_len = block_len * length;
        let negative = unsigned & (1 << (bit_len - 1)) != 0;

        let int = (unsigned & !(1 << (bit_len - 1))) as i128;

        if negative {
            Ok(-int)
        } else {
            Ok(int)
        }
    }

    fn read_vixler(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxle(s);
        let result = parse_vuxr(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        let block_len = size * 8 - 1;
        let bit_len = block_len * length;
        let negative = unsigned & (1 << (bit_len - 1)) != 0;

        let int = (unsigned & !(1 << (bit_len - 1))) as i128;

        if negative {
            Ok(-int)
        } else {
            Ok(int)
        }
    }

    fn read_vixber(&mut self, size: u8) -> Result<i128> {
        let mut fun = |s: u8| self.read_uxbe(s);
        let result = parse_vuxr(&mut fun, size)?;
        let unsigned = result.0;
        let length = result.1 as u8;

        let block_len = size * 8 - 1;
        let bit_len = block_len * length;
        let negative = unsigned & (1 << (bit_len - 1)) != 0;

        let int = (unsigned & !(1 << (bit_len - 1))) as i128;

        if negative {
            Ok(-int)
        } else {
            Ok(int)
        }
    }
}
