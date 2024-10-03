//! ## Create a data editor
//!
//! ### References (recommended)
//!
//! - [`read_ref`][crate::data::read_ref] creates a reader from a [`Vec<u8>`] reference.
//! - [`write_ref`][crate::data::write_ref] creates a writer from a mutable [`Vec<u8>`] reference.
//! - [`rw_ref`][crate::data::rw_ref] creates a reader and writer from a mutable [`Vec<u8>`] reference.
//!
//! ### Owned data
//!
//! - [`read`][crate::data::read] creates a reader from a [`Vec<u8>`].
//! - [`write`][crate::data::write] creates a writer from a [`Vec<u8>`].
//! - [`rw`][crate::data::rw] creates a reader and writer from a [`Vec<u8>`].
//!
//! ## Close a data editor and get the data
//!
//! - [`close_ref`][crate::data::close_ref] closes a reader, writer, or reader and writer and returns the reference to the data.
//! - [`close_mut`][crate::data::close_mut] closes a writer or reader and writer and returns the mutable reference to the data.
//! - [`close`][crate::data::close] closes a reader, writer, or reader and writer and returns the data.

#![allow(rustdoc::broken_intra_doc_links)] // rustdoc has some issues with the links above
use crate::{DataType, Readable, Rw, Seekable, Writable};
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

mod r#ref;
pub use r#ref::{
    close as close_ref, close_mut, read as read_ref, rw as rw_ref, write as write_ref,
    ClosableMutData, ClosableRefData, RRefData, RwRefData, WRefData,
};

/// A [`Vec<u8>`] reader.
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

impl Seekable for RData {}

impl<'a> Readable<'a> for RData {
    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::Vec(self.data)))
    }
}

/// A [`Vec<u8>`] writer.
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

impl Seekable for WData {}

impl<'a> Writable<'a> for WData {
    fn alloc(&mut self, len: u64) -> Result<()> {
        self.data.resize(len as usize, 0);
        Ok(())
    }

    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>>
    where
        Self: 'a,
    {
        Ok(Some(DataType::Vec(self.data)))
    }
}

/// A [`Vec<u8>`] reader and writer.
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

impl Seekable for RwData {}

impl<'a> Readable<'a> for RwData {
    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        Ok(Some(DataType::Vec(self.data)))
    }
}

impl<'a> Writable<'a> for RwData {
    fn alloc(&mut self, len: u64) -> Result<()> {
        self.data.resize(len as usize, 0);
        Ok(())
    }

    fn lock(&mut self, _: bool) -> Result<()> {
        Ok(())
    }

    fn unlock(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(self) -> Result<Option<DataType<'a>>>
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

/// Enumerates all the possible data types that can be passed into the [`close`] function.
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

/// Creates a new reader from a [`Vec<u8>`].
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let mut reader = dh::data::read(data);
///
/// assert_eq!(reader.read_u8_at(0).unwrap(), 0);
///
/// let data = dh::data::close(reader); // gets the data back
/// ```
pub fn read(data: Vec<u8>) -> RData {
    RData { data, pos: 0 }
}

/// Creates a new writer from a [`Vec<u8>`].
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut writer = dh::data::write(vec![0; 2]);
///
/// writer.write_u16be(0xabcd).unwrap(); // sets the first byte
///
/// let data = dh::data::close(writer); // gets the data back
/// assert_eq!(data, vec![0xab, 0xcd]);
/// ```
pub fn write(data: Vec<u8>) -> WData {
    WData { data, pos: 0 }
}

/// Creates an empty [`Vec<u8>`] writer.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut writer = dh::data::write_empty();
///
/// writer.write_utf8_at(0, &"Hello, world!".to_owned()).unwrap();
///
/// let data = dh::data::close(writer); // gets the data back
/// assert_eq!(data, "Hello, world!".as_bytes());
/// ```
pub fn write_empty() -> WData {
    WData {
        data: Vec::new(),
        pos: 0,
    }
}

/// Creates a new [`Vec<u8>`] writer with a specific length.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut writer = dh::data::write_new(13);
///
/// writer.write_utf8_at(0, &"Hello, world!".to_owned()).unwrap();
///
/// let data = dh::data::close(writer); // gets the data back
/// assert_eq!(data, "Hello, world!".as_bytes());
/// ```
pub fn write_new(len: u64) -> WData {
    WData {
        data: vec![0; len as usize],
        pos: 0,
    }
}

/// Creates a new reader and writer from a [`Vec<u8>`].
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let mut rw = dh::data::rw(data);
///
/// rw.write_u8(8).unwrap();
///
/// assert_eq!(rw.read_bytes_at(0, 8).unwrap(), vec![8, 1, 2, 3, 4, 5, 6, 7]);
/// rw.rw_close(); // we don't need the data back
/// ```
pub fn rw(data: Vec<u8>) -> RwData {
    RwData { data, pos: 0 }
}

/// Creates an empty [`Vec<u8>`] reader and writer.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut rw = dh::data::rw_empty();
///
/// rw.write_utf8_at(0, &"Hello, world!".to_string()).unwrap();
/// assert_eq!(rw.read_utf8(13).unwrap(), "Hello, world!");
///
/// rw.rw_close(); // we don't need the data back
/// ```
pub fn rw_empty() -> RwData {
    RwData {
        data: Vec::new(),
        pos: 0,
    }
}

/// Creates a new [`Vec<u8>`] reader and writer with a specific length.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut rw = dh::data::rw_new(13);
///
/// rw.write_utf8_at(0, &"Hello, world!".to_string()).unwrap();
/// assert_eq!(rw.read_utf8(13).unwrap(), "Hello, world!");
///
/// rw.rw_close(); // we don't need the data back
/// ```
pub fn rw_new(len: u64) -> RwData {
    RwData {
        data: vec![0; len as usize],
        pos: 0,
    }
}

/// Closes a reader, writer, or reader and writer and returns the data the structure was holding.
///
/// **Note**: This function only takes [`RData`], [`WData`], and [`RwData`] as input.
///
/// For references, please use the [`close_ref`] and [`close_mut`] functions.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
/// let clone = data.clone();
/// let mut reader = dh::data::read(data);
///
/// // do something with the reader
///
/// let data = dh::data::close(reader); // gets the data back
/// assert_eq!(data, clone);
/// ```
pub fn close<T: Into<ClosableData>>(closable: T) -> Vec<u8> {
    // these unwraps are safe because the data is always returned
    match match closable.into() {
        ClosableData::R(r) => r.close().unwrap().unwrap(),
        ClosableData::W(w) => w.close().unwrap().unwrap(),
        ClosableData::Rw(rw) => rw.rw_close().unwrap().unwrap(),
    } {
        DataType::Vec(data) => data,
        _ => unreachable!(),
    }
}
