use crate::{DataType, Readable, Rw, Writable};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

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

pub struct WData {
    data: Vec<u8>,
    pos: usize,
}

impl Write for WData {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        let data_len = self.data.len();
        let pos = self.pos;
        if pos > data_len {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid write position",
            ));
        }
        if pos == data_len {
            self.data.extend_from_slice(buf);
        } else {
            let end = pos + len;
            if end > data_len {
                self.data.resize(end, 0);
            }
            self.data[pos..end].copy_from_slice(buf);
        }
        self.pos += len;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for WData {
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

impl Writable for WData {
    fn alloc(&mut self, len: u64) -> Result<()> {
        self.data.resize(len as usize, 0);
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close<'a>(self) -> Result<Option<DataType<'a>>>
    where
        Self: 'a,
    {
        Ok(Some(DataType::Vec(self.data)))
    }
}

pub struct RwData {
    data: Vec<u8>,
    pos: usize,
}

impl Read for RwData {
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

impl Write for RwData {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        let data_len = self.data.len();
        let pos = self.pos;
        if pos > data_len {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "invalid write position",
            ));
        }
        if pos == data_len {
            self.data.extend_from_slice(buf);
        } else {
            let end = pos + len;
            if end > data_len {
                self.data.resize(end, 0);
            }
            self.data[pos..end].copy_from_slice(buf);
        }
        self.pos += len;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for RwData {
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

impl<'a> Readable<'a> for RwData {
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

impl Writable for RwData {
    fn alloc(&mut self, len: u64) -> Result<()> {
        self.data.resize(len as usize, 0);
        Ok(())
    }

    fn lock(&mut self) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close<'a>(self) -> Result<Option<DataType<'a>>>
    where
        Self: 'a,
    {
        Ok(Some(DataType::Vec(self.data)))
    }
}

impl<'a> Rw<'a> for RwData {
    fn rw_close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::Vec(self.data)))
    }
}

pub enum ClosableData {
    R(RData),
    W(WData),
    Rw(RwData),
}

impl From<RData> for ClosableData {
    fn from(r: RData) -> Self {
        ClosableData::R(r)
    }
}

impl From<WData> for ClosableData {
    fn from(w: WData) -> Self {
        ClosableData::W(w)
    }
}

impl From<RwData> for ClosableData {
    fn from(rw: RwData) -> Self {
        ClosableData::Rw(rw)
    }
}

pub fn read(data: Vec<u8>) -> RData {
    RData { data, pos: 0 }
}

pub fn write(data: Vec<u8>) -> WData {
    WData { data, pos: 0 }
}

pub fn write_empty() -> WData {
    WData {
        data: Vec::new(),
        pos: 0,
    }
}

pub fn write_new(len: u64) -> WData {
    WData {
        data: vec![0; len as usize],
        pos: 0,
    }
}

pub fn rw(data: Vec<u8>) -> RwData {
    RwData { data, pos: 0 }
}

pub fn rw_empty() -> RwData {
    RwData {
        data: Vec::new(),
        pos: 0,
    }
}

pub fn rw_new(len: u64) -> RwData {
    RwData {
        data: vec![0; len as usize],
        pos: 0,
    }
}

pub fn close<T: Into<ClosableData>>(closable: T) -> Result<Vec<u8>> {
    match closable.into() {
        ClosableData::R(r) => match r.close()? {
            Some(DataType::Vec(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
        ClosableData::W(w) => match w.close()? {
            Some(DataType::Vec(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
        ClosableData::Rw(rw) => match rw.rw_close()? {
            Some(DataType::Vec(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
    }
}
