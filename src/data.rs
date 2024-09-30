use crate::{DataType, Readable};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

mod r#ref;
pub use r#ref::{close as close_ref, read as read_ref, ClosableRefData, RRefData};

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

impl<'a> Readable<'a> for RData {
    fn lock(&mut self) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::Vec(self.data)))
    }
}

pub enum ClosableData {
    R(RData),
}

impl From<RData> for ClosableData {
    fn from(r: RData) -> Self {
        ClosableData::R(r)
    }
}

pub fn read(data: Vec<u8>) -> RData {
    RData { data, pos: 0 }
}

pub fn close<T: Into<ClosableData>>(closable: T) -> Result<Vec<u8>> {
    match closable.into() {
        ClosableData::R(r) => match r.close()? {
            Some(DataType::Vec(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
    }
}
