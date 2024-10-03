use std::io::{Read, Result, Seek, SeekFrom};

use crate::{Readable, Seekable};

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
