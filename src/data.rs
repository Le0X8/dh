use crate::Readable;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

pub struct RData {
    data: Vec<u8>,
    pos: usize,
}

impl Read for RData {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();
        let data_len = self.data.len();
        let pos = self.pos;
        if pos >= data_len {
            return Ok(0);
        }
        let end = pos + len;
        let end = if end > data_len { data_len } else { end };
        let len = end - pos;
        buf[..len].copy_from_slice(&self.data[pos..end]);
        self.pos = end;
        Ok(len)
    }
}

impl Seek for RData {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        use SeekFrom::*;
        let pos = match pos {
            Start(pos) => pos as i64,
            End(pos) => self.data.len() as i64 + pos,
            Current(pos) => self.pos as i64 + pos,
        };
        if pos < 0 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid seek to a negative position",
            ));
        }
        self.pos = pos as usize;
        Ok(pos as u64)
    }
}

impl Readable for RData {
    fn lock(&mut self) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<()> {
        Ok(())
    }
}

pub fn from_r(data: Vec<u8>) -> RData {
    RData { data, pos: 0 }
}
