//! ## Open a file
//! - [`open_r`][crate::file::open_r] opens a file in read-only mode.
//! - [`open_w`][crate::file::open_w] opens a file in write-only mode.
//! - [`open_rw`][crate::file::open_rw] opens a file in read-write mode.

use crate::{DataType, Readable, Rw, Seekable, Writable};
use fs4::fs_std::FileExt;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, Write},
    path::Path,
};

/// A file reader.
pub struct RFile {
    file: File,
}

impl Read for RFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Seek for RFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}

impl Drop for RFile {
    fn drop(&mut self) {
        self.file.unlock().unwrap();
        self.file.sync_all().unwrap();
    }
}

impl Seekable for RFile {}

impl<'a> Readable<'a> for RFile {
    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        if block {
            self.file.lock_exclusive()
        } else {
            self.file.try_lock_exclusive()
        }
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

/// Opens a file in read-only mode.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut reader = dh::file::open_r("tests/samples/000").unwrap();
///
/// let size = reader.size().unwrap();
/// assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");
///
/// reader.close().unwrap();
/// ```
pub fn open_r<P>(path: P) -> Result<RFile>
where
    P: AsRef<Path>,
{
    Ok(RFile {
        file: match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        },
    })
}

/// A file writer.
pub struct WFile {
    file: File,
}

impl Write for WFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

impl Seek for WFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}

impl Drop for WFile {
    fn drop(&mut self) {
        self.file.unlock().unwrap();
        self.file.sync_all().unwrap();
    }
}

impl Seekable for WFile {}

impl<'a> Writable<'a> for WFile {
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn alloc(&mut self, len: u64) -> Result<()> {
        match self.file.allocate(len) {
            Ok(_) => Ok(()),
            Err(_) => match self.file.set_len(len) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        if block {
            self.file.lock_exclusive()
        } else {
            self.file.try_lock_exclusive()
        }
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;

        Ok(None)
    }
}

/// Opens a file in write-only mode.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut writer = dh::file::open_w("doctest-file-open_w").unwrap();
/// writer.write_utf8(&"Hello, world!".to_string()).unwrap();
///
/// writer.close().unwrap();
/// ```
pub fn open_w<P>(path: P) -> Result<WFile>
where
    P: AsRef<Path>,
{
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
    {
        Ok(file) => Ok(WFile { file }),
        Err(e) => Err(e),
    }
}

/// A file reader and writer.
pub struct RwFile {
    file: File,
}

impl Read for RwFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf)
    }
}

impl Write for RwFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.file.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.file.flush()
    }
}

impl Seek for RwFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        self.file.seek(pos)
    }
}

impl Drop for RwFile {
    fn drop(&mut self) {
        self.file.unlock().unwrap();
        self.file.sync_all().unwrap();
    }
}

impl Seekable for RwFile {}

impl<'a> Readable<'a> for RwFile {
    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        if block {
            self.file.lock_exclusive()
        } else {
            self.file.try_lock_exclusive()
        }
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

impl<'a> Writable<'a> for RwFile {
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn alloc(&mut self, len: u64) -> Result<()> {
        match self.file.allocate(len) {
            Ok(_) => Ok(()),
            Err(_) => match self.file.set_len(len) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        if block {
            self.file.lock_exclusive()
        } else {
            self.file.try_lock_exclusive()
        }
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

impl<'a> Rw<'a> for RwFile {
    fn rw_as_trait(&mut self) -> &mut dyn Rw<'a> {
        self
    }

    fn rw_close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

/// Opens a file in read-write mode.
///
/// ### Example
///
/// ```rust
/// use dh::recommended::*;
///
/// let mut rw = dh::file::open_rw("doctest-file-open_rw").unwrap();
///
/// rw.write_utf8(&"Hello, world!".to_string()).unwrap();
/// rw.rewind().unwrap();
/// assert_eq!(rw.read_utf8(13).unwrap(), "Hello, world!");
/// ```
pub fn open_rw<P>(path: P) -> Result<RwFile>
where
    P: AsRef<Path>,
{
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
    {
        Ok(file) => Ok(RwFile { file }),
        Err(e) => Err(e),
    }
}
