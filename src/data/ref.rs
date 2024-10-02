use crate::{DataType, Readable, Rw, Seekable, Writable};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

/// A [`Vec<u8>`] reference reader.
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

impl Seekable for RRefData<'_> {}

impl<'a> Readable<'a> for RRefData<'a> {
    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecRef(self.data)))
    }
}

/// A [`Vec<u8>`] reference writer.
pub struct WRefData<'a> {
    data: &'a mut Vec<u8>,
    pos: usize,
}

impl Write for WRefData<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        let data_len = self.data.len();
        let pos = self.pos;
        if pos >= data_len {
            return Ok(0);
        }
        let end = pos + len;
        let end = if end > data_len { data_len } else { end };
        let len = end - pos;
        self.data[pos..end].copy_from_slice(buf);
        self.pos = end;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for WRefData<'_> {
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

impl Seekable for WRefData<'_> {}

impl<'a> Writable<'a> for WRefData<'a> {
    fn alloc(&mut self, len: u64) -> Result<()> {
        let data_len = self.data.len();
        if len > data_len as u64 {
            self.data.resize(len as usize, 0);
        }
        Ok(())
    }

    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }
}

/// A [`Vec<u8>`] reference reader and writer.
pub struct RwRefData<'a> {
    data: &'a mut Vec<u8>,
    pos: usize,
}

impl Read for RwRefData<'_> {
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

impl Write for RwRefData<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len();
        let data_len = self.data.len();
        let pos = self.pos;
        if pos >= data_len {
            return Ok(0);
        }
        let end = pos + len;
        let end = if end > data_len { data_len } else { end };
        let len = end - pos;
        self.data[pos..end].copy_from_slice(buf);
        self.pos = end;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for RwRefData<'_> {
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

impl Seekable for RwRefData<'_> {}

impl<'a> Readable<'a> for RwRefData<'a> {
    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }
}

impl<'a> Writable<'a> for RwRefData<'a> {
    fn alloc(&mut self, len: u64) -> Result<()> {
        let data_len = self.data.len();
        if len > data_len as u64 {
            self.data.resize(len as usize, 0);
        }
        Ok(())
    }

    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }
}

impl<'a> Rw<'a> for RwRefData<'a> {
    fn rw_close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }
}

/// Enumerates all the possible data types that can be passed into the [`close_ref`][close] function.
pub enum ClosableRefData<'a> {
    R(RRefData<'a>),
    W(WRefData<'a>),
    Rw(RwRefData<'a>),
}

impl<'a> From<RRefData<'a>> for ClosableRefData<'a> {
    fn from(r: RRefData<'a>) -> Self {
        ClosableRefData::R(r)
    }
}

impl<'a> From<WRefData<'a>> for ClosableRefData<'a> {
    fn from(w: WRefData<'a>) -> Self {
        ClosableRefData::W(w)
    }
}

impl<'a> From<RwRefData<'a>> for ClosableRefData<'a> {
    fn from(rw: RwRefData<'a>) -> Self {
        ClosableRefData::Rw(rw)
    }
}

/// Enumerates all the possible data types that can be passed into the [`close_mut`] function.
pub enum ClosableMutData<'a> {
    W(WRefData<'a>),
    Rw(RwRefData<'a>),
}

impl<'a> From<WRefData<'a>> for ClosableMutData<'a> {
    fn from(w: WRefData<'a>) -> Self {
        ClosableMutData::W(w)
    }
}

impl<'a> From<RwRefData<'a>> for ClosableMutData<'a> {
    fn from(rw: RwRefData<'a>) -> Self {
        ClosableMutData::Rw(rw)
    }
}

/// Creates a new reader from a [`Vec<u8>`] reference.
pub fn read(data: &Vec<u8>) -> RRefData {
    RRefData { data, pos: 0 }
}

/// Creates a new writer from a [`Vec<u8>`] reference.
pub fn write(data: &mut Vec<u8>) -> WRefData {
    WRefData { data, pos: 0 }
}

/// Creates a new reader and writer from a [`Vec<u8>`] reference.
pub fn rw(data: &mut Vec<u8>) -> RwRefData {
    RwRefData { data, pos: 0 }
}

/// Closes a reader, writer, or reader and writer and returns the reference to the data it refers to.
///
/// To get the mutable reference to the data, use the [`close_mut`] function instead.
pub fn close<'a, T: Into<ClosableRefData<'a>>>(closable: T) -> Result<&'a Vec<u8>> {
    match closable.into() {
        ClosableRefData::R(r) => match r.close()? {
            Some(DataType::VecRef(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
        ClosableRefData::W(w) => match w.close()? {
            Some(DataType::VecMut(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
        ClosableRefData::Rw(rw) => match rw.rw_close()? {
            Some(DataType::VecMut(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
    }
}

/// Closes a writer or reader and writer and returns the mutable reference to the data it refers to.
///
/// To get the immutable reference to the data, use the [`close_ref`][close] function instead.
pub fn close_mut<'a, T: Into<ClosableMutData<'a>>>(closable: T) -> Result<&'a mut Vec<u8>> {
    match closable.into() {
        ClosableMutData::W(w) => match w.close()? {
            Some(DataType::VecMut(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
        ClosableMutData::Rw(rw) => match rw.rw_close()? {
            Some(DataType::VecMut(data)) => Ok(data),
            _ => Err(Error::new(ErrorKind::InvalidData, "invalid data type")),
        },
    }
}
