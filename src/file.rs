use crate::{DataType, Readable, Rw, Writable};
use fs4::fs_std::FileExt;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, Write},
    path::Path,
};

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

impl<'a> Readable<'a> for RFile {
    fn lock(&mut self) -> Result<()> {
        self.file.lock_exclusive()
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

pub struct WFile {
    file: File,
}

impl WFile {}

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

impl Writable for WFile {
    fn alloc(&mut self, len: u64) -> Result<()> {
        match self.file.allocate(len) {
            Ok(_) => Ok(()),
            Err(_) => match self.file.set_len(len) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }

    fn lock(&mut self) -> Result<()> {
        self.file.lock_exclusive()
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close<'a>(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;

        Ok(None)
    }
}

pub fn open_w<P>(path: P) -> Result<WFile>
where
    P: AsRef<Path>,
{
    match File::create(path) {
        Ok(file) => Ok(WFile { file }),
        Err(e) => Err(e),
    }
}

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

impl<'a> Readable<'a> for RwFile {
    fn lock(&mut self) -> Result<()> {
        self.file.lock_exclusive()
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

impl Writable for RwFile {
    fn alloc(&mut self, len: u64) -> Result<()> {
        match self.file.allocate(len) {
            Ok(_) => Ok(()),
            Err(_) => match self.file.set_len(len) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
        }
    }

    fn lock(&mut self) -> Result<()> {
        self.file.lock_exclusive()
    }

    fn unlock(&mut self) -> Result<()> {
        self.file.unlock()
    }

    fn close<'a>(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

impl<'a> Rw<'a> for RwFile {
    fn rw_close(self) -> Result<Option<DataType<'a>>> {
        self.file.unlock()?;
        self.file.sync_all()?;
        Ok(None)
    }
}

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
