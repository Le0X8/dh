use std::io::{Read, Result, Seek, SeekFrom, Write};

use crate::{Readable, Rw, Seekable, Source, Writable};

/// A limited reader.
///
/// See [`Readable::limit`] for more information.
pub struct RLimited<'a> {
    data: &'a mut dyn Readable<'a>,
    start: u64,
    end: u64,
}

impl<'a> RLimited<'a> {
    /// Gets the reference back.
    /// This can be useful if you run into borrow checker issues.
    pub fn unlimit(self) -> &'a mut dyn Readable<'a> {
        self.data
    }
}

impl<'a> Read for RLimited<'a> {
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

impl<'a> Seek for RLimited<'a> {
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

impl<'a> Seekable for RLimited<'a> {}

impl<'a> Readable<'a> for RLimited<'a> {
    fn source(&mut self) -> Source {
        self.data.source()
    }

    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }
}

pub(crate) fn limit_r<'a>(
    data: &'a mut dyn Readable<'a>,
    start: u64,
    length: u64,
) -> Result<RLimited<'a>> {
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
pub struct WLimited<'a> {
    data: &'a mut dyn Writable<'a>,
    start: u64,
    end: u64,
}

impl<'a> WLimited<'a> {
    /// Gets the reference back.
    /// This can be useful if you run into borrow checker issues.
    pub fn unlimit(self) -> &'a mut dyn Writable<'a> {
        self.data
    }
}

impl<'a> Write for WLimited<'a> {
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

impl<'a> Seek for WLimited<'a> {
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

impl<'a> Seekable for WLimited<'a> {}

impl<'a> Writable<'a> for WLimited<'a> {
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }

    fn source(&mut self) -> Source {
        self.data.source()
    }
}

pub(crate) fn limit_w<'a>(
    data: &'a mut dyn Writable<'a>,
    start: u64,
    length: u64,
) -> Result<WLimited<'a>> {
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
pub struct RwLimited<'a> {
    data: &'a mut dyn Rw<'a>,
    start: u64,
    end: u64,
}

impl<'a> RwLimited<'a> {
    /// Gets the reference back.
    /// This can be useful if you run into borrow checker issues.
    pub fn unlimit(self) -> &'a mut dyn Rw<'a> {
        self.data
    }
}

impl<'a> Read for RwLimited<'a> {
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

impl<'a> Write for RwLimited<'a> {
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

impl<'a> Seek for RwLimited<'a> {
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

impl<'a> Seekable for RwLimited<'a> {}

impl<'a> Readable<'a> for RwLimited<'a> {
    fn as_trait(&mut self) -> &mut dyn Readable<'a> {
        self
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }

    fn source(&mut self) -> Source {
        self.data.rw_source()
    }
}

impl<'a> Writable<'a> for RwLimited<'a> {
    fn as_trait(&mut self) -> &mut dyn Writable<'a> {
        self
    }

    fn close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }

    fn source(&mut self) -> Source {
        self.data.rw_source()
    }
}

impl<'a> Rw<'a> for RwLimited<'a> {
    fn rw_as_trait(&mut self) -> &mut dyn Rw<'a> {
        self
    }

    fn rw_close(self) -> Result<Option<crate::DataType<'a>>> {
        Ok(None)
    }

    fn rw_source(&mut self) -> Source {
        self.data.rw_source()
    }
}

pub(crate) fn limit_rw<'a>(
    data: &'a mut dyn Rw<'a>,
    start: u64,
    length: u64,
) -> Result<RwLimited<'a>> {
    data.to(start)?;
    Ok(RwLimited {
        data,
        start,
        end: start + length,
    })
}
