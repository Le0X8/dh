use std::io::{Read, Result, Seek, SeekFrom, Write};

use crate::{Readable, Rw, Seekable, Writable};

/// A limited reader.
///
/// See [`Readable::limit`] for more information.
pub struct RLimited<'a, T: Readable<'a> + ?Sized> {
    data: &'a mut T,
    start: u64,
    end: u64,
}

impl<'a, T: Readable<'a>> Read for RLimited<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let start_pos = self.data.pos()?;
        let end_pos = start_pos + buf.len() as u64;

        if start_pos < self.start || end_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "reading out of bounds",
            ));
        }

        self.data.read(buf)
    }
}

impl<'a, T: Readable<'a>> Seek for RLimited<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => self.start + offset,
            SeekFrom::End(offset) => (self.end as i128 + offset as i128) as u64,
            SeekFrom::Current(offset) => {
                let current = self.data.pos()?;
                (current as i128 + offset as i128) as u64
            }
        };

        if new_pos < self.start || new_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "new position is out of bounds",
            ));
        }

        Ok(self.data.to(new_pos)? - self.start)
    }
}

impl<'a, T: Readable<'a>> Seekable for RLimited<'a, T> {}

impl<'a, T: Readable<'a>> Readable<'a> for RLimited<'a, T> {
    fn lock(&mut self, block: bool) -> Result<()> {
        self.data.lock(block)
    }

    fn unlock(&mut self) -> Result<()> {
        self.data.unlock()
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

pub(crate) fn limit_r<'a, T: Readable<'a> + ?Sized>(
    data: &'a mut T,
    start: u64,
    length: u64,
) -> Result<RLimited<'a, T>> {
    data.to(start)?;
    Ok(RLimited {
        data,
        start,
        end: start + length,
    })
}

/// A limited writer.
///
/// See [`Writable::limit`] for more information.
pub struct WLimited<'a, T: Writable<'a> + ?Sized> {
    data: &'a mut T,
    start: u64,
    end: u64,
}

impl<'a, T: Writable<'a>> Write for WLimited<'a, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let start_pos = self.data.pos()?;
        let end_pos = start_pos + buf.len() as u64;

        if start_pos < self.start || end_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "writing out of bounds",
            ));
        }

        self.data.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}

impl<'a, T: Writable<'a>> Seek for WLimited<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => self.start + offset,
            SeekFrom::End(offset) => (self.end as i128 + offset as i128) as u64,
            SeekFrom::Current(offset) => {
                let current = self.data.pos()?;
                (current as i128 + offset as i128) as u64
            }
        };

        if new_pos < self.start || new_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "new position is out of bounds",
            ));
        }

        Ok(self.data.to(new_pos)? - self.start)
    }
}

impl<'a, T: Writable<'a>> Seekable for WLimited<'a, T> {}

impl<'a, T: Writable<'a>> Writable<'a> for WLimited<'a, T> {
    fn alloc(&mut self, _: u64) -> Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "alloc is not supported for limited writers",
        ))
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        self.data.lock(block)
    }

    fn unlock(&mut self) -> Result<()> {
        self.data.unlock()
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

pub(crate) fn limit_w<'a, T: Writable<'a> + ?Sized>(
    data: &'a mut T,
    start: u64,
    length: u64,
) -> Result<WLimited<'a, T>> {
    data.to(start)?;
    Ok(WLimited {
        data,
        start,
        end: start + length,
    })
}

/// A limited R/W stream.
///
/// See [`Rw::rw_limit`] for more information.
pub struct RwLimited<'a, T: Rw<'a> + ?Sized> {
    data: &'a mut T,
    start: u64,
    end: u64,
}

impl<'a, T: Rw<'a>> Read for RwLimited<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let start_pos = self.data.pos()?;
        let end_pos = start_pos + buf.len() as u64;

        if start_pos < self.start || end_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "reading out of bounds",
            ));
        }

        self.data.read(buf)
    }
}

impl<'a, T: Rw<'a>> Write for RwLimited<'a, T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let start_pos = self.data.pos()?;
        let end_pos = start_pos + buf.len() as u64;

        if start_pos < self.start || end_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "writing out of bounds",
            ));
        }

        self.data.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}

impl<'a, T: Rw<'a>> Seek for RwLimited<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => self.start + offset,
            SeekFrom::End(offset) => (self.end as i128 + offset as i128) as u64,
            SeekFrom::Current(offset) => {
                let current = self.data.pos()?;
                (current as i128 + offset as i128) as u64
            }
        };

        if new_pos < self.start || new_pos > self.end {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "new position is out of bounds",
            ));
        }

        Ok(self.data.to(new_pos)? - self.start)
    }
}

impl<'a, T: Rw<'a>> Seekable for RwLimited<'a, T> {}

impl<'a, T: Rw<'a>> Readable<'a> for RwLimited<'a, T> {
    fn lock(&mut self, block: bool) -> Result<()> {
        Readable::lock(self.data, block)
    }

    fn unlock(&mut self) -> Result<()> {
        Readable::unlock(self.data)
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

impl<'a, T: Rw<'a>> Writable<'a> for RwLimited<'a, T> {
    fn alloc(&mut self, _: u64) -> Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "alloc is not supported for limited writers",
        ))
    }

    fn lock(&mut self, block: bool) -> Result<()> {
        Writable::lock(self.data, block)
    }

    fn unlock(&mut self) -> Result<()> {
        Writable::unlock(self.data)
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

impl<'a, T: Rw<'a>> Rw<'a> for RwLimited<'a, T> {
    fn rw_close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

pub(crate) fn limit_rw<'a, T: Rw<'a> + ?Sized>(
    data: &'a mut T,
    start: u64,
    length: u64,
) -> Result<RwLimited<'a, T>> {
    data.to(start)?;
    Ok(RwLimited {
        data,
        start,
        end: start + length,
    })
}
