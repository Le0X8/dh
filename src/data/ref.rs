use crate::{DataType, Readable};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

pub struct RRefData<'a> {
    data: &'a Vec<u8>,
    pos: usize,
}

impl Read for RRefData<'_> {
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

impl Seek for RRefData<'_> {
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

impl<'a> Readable<'a> for RRefData<'a> {
    fn lock(&mut self) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecRef(self.data)))
    }
}

pub enum ClosableRefData<'a> {
    R(RRefData<'a>),
}

impl<'a> From<RRefData<'a>> for ClosableRefData<'a> {
    fn from(r: RRefData<'a>) -> Self {
        ClosableRefData::R(r)
    }
}

pub fn read(data: &Vec<u8>) -> RRefData {
    RRefData { data, pos: 0 }
}

pub fn close<'a, T: Into<ClosableRefData<'a>>>(closable: T) -> Result<&'a Vec<u8>> {
    match closable.into() {
        ClosableRefData::R(r) => match r.close()? {
            Some(DataType::VecRef(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
    }
}
