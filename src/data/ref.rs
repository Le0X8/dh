use crate::{DataType, Readable, Rw, Seekable, Source, Writable};
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
    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecRef(self.data)))
    }

    fn source(&mut self) -> Source {
        Source::VecRef(self.data)
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
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }

    fn source(&mut self) -> Source {
        Source::Vec(self.data)
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
    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }

    fn source(&mut self) -> Source {
        Source::Vec(self.data)
    }
}

impl<'a> Writable<'a> for RwRefData<'a> {
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }

    fn source(&mut self) -> Source {
        Source::Vec(self.data)
    }
}

impl<'a> Rw<'a> for RwRefData<'a> {
    fn rw_as_trait(&mut self) -> &mut dyn Rw<'a> {
        self
    }

    fn rw_close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::VecMut(self.data)))
    }

    fn rw_source(&mut self) -> Source {
        Source::Vec(self.data)
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
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let mut reader = dh::data::read_ref(&data);
///
/// assert_eq!(reader.read_u8_at(0).unwrap(), 0);
/// ```
pub fn read(data: &Vec<u8>) -> RRefData {
    RRefData { data, pos: 0 }
}

/// Creates a new writer from a [`Vec<u8>`] reference.
///
/// ### Example
/// ```rust
/// use dh::recommended::*;
///
/// let mut data = "Hello, world!".as_bytes().to_vec();
/// let mut writer = dh::data::write_ref(&mut data);
///
/// writer.write_utf8_at(7, &"Rust!".to_string()).unwrap();
/// writer.close().unwrap();
///
/// assert_eq!(data, "Hello, Rust!!".as_bytes());
/// ```
pub fn write(data: &mut Vec<u8>) -> WRefData {
    WRefData { data, pos: 0 }
}

/// Creates a new reader and writer from a [`Vec<u8>`] reference.
///
/// ### Example
/// ```rust
/// use dh::recommended::*;
///
/// let mut data = vec![0u8; 2];
/// let mut rw = dh::data::rw_ref(&mut data);
///
/// rw.write_u16be(0x1234).unwrap();
/// assert_eq!(rw.read_u16be_at(0).unwrap(), 0x1234);
///
/// rw.rw_close().unwrap(); // removes the mutable reference
/// ```
pub fn rw(data: &mut Vec<u8>) -> RwRefData {
    RwRefData { data, pos: 0 }
}

/// Closes a reader, writer, or reader and writer and returns the reference to the data it refers to.
///
/// **Note**: This function only takes [`RRefData`], [`WRefData`], and [`RwRefData`] as input.
///
/// To get the mutable reference to the data, use the [`close_mut`] function instead.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let data = "Hello, world!".as_bytes().to_vec();
/// let mut reader = dh::data::read_ref(&data);
///
/// let size = reader.size().unwrap();
/// assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");
///
/// assert_eq!(dh::data::close_ref(reader), &data);
/// ```
pub fn close<'a, T: Into<ClosableRefData<'a>>>(closable: T) -> &'a Vec<u8> {
    // these unwraps are safe because the data is always returned
    match match closable.into() {
        ClosableRefData::R(r) => r.close().unwrap().unwrap(),
        ClosableRefData::W(w) => w.close().unwrap().unwrap(),
        ClosableRefData::Rw(rw) => rw.rw_close().unwrap().unwrap(),
    } {
        DataType::VecRef(data) => data,
        DataType::VecMut(data) => data,
        _ => unreachable!(),
    }
}

/// Closes a writer or reader and writer and returns the mutable reference to the data it refers to.
///
/// **Note**: This function only takes [`WRefData`] and [`RwRefData`] as input.
///
/// To get the immutable reference to the data, use the [`close_ref`][close] function instead.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut data = "Hello, world!".as_bytes().to_vec();
/// let mut writer = dh::data::write_ref(&mut data);
///
/// writer.write_utf8_at(7, &"Rust!".to_string()).unwrap();
/// let data_ref = dh::data::close_mut(writer);
///
/// assert_eq!(data_ref, &mut "Hello, Rust!!".as_bytes().to_vec());
/// ```
pub fn close_mut<'a, T: Into<ClosableMutData<'a>>>(closable: T) -> &'a mut Vec<u8> {
    // these unwraps are safe because the data is always returned
    match match closable.into() {
        ClosableMutData::W(w) => w.close().unwrap().unwrap(),
        ClosableMutData::Rw(rw) => rw.rw_close().unwrap().unwrap(),
    } {
        DataType::VecMut(data) => data,
        _ => unreachable!(),
    }
}
