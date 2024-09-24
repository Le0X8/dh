use std::io::{Result, Seek, Write};

pub trait Writable
where
    Self: Write + Seek,
{
    fn alloc(&mut self, len: u64) -> Result<()>;
    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;
    fn close(self) -> Result<()>;

    fn rewind(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(0))
    }

    fn end(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::End(0))
    }

    fn to(&mut self, pos: u64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Start(pos))
    }

    fn jump(&mut self, pos: i64) -> Result<u64> {
        self.seek(std::io::SeekFrom::Current(pos))
    }

    fn size(&mut self) -> Result<u64> {
        let pos_before = self.stream_position()?;
        let size = self.end();
        self.to(pos_before)?;
        size
    }

    fn write_utf8_at(&mut self, pos: u64, s: &String) -> Result<()> {
        let pos_before = self.stream_position()?;
        self.to(pos)?;
        self.write_utf8(s)?;
        self.to(pos_before)?;
        Ok(())
    }

    fn write_utf8(&mut self, s: &String) -> Result<()> {
        self.write_all(s.as_bytes())
    }
}
