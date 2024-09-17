use std::io::{Result, Seek, Write};

pub trait Writable
where
    Self: Write + Seek,
{
    fn alloc(&mut self, len: u64) -> Result<()>;
    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;
    fn close(self) -> Result<()>;

    fn write_utf8_at(&mut self, s: &String, pos: u64) -> Result<()> {
        match self.seek(std::io::SeekFrom::Start(pos)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        self.write_all(s.as_bytes())
    }
}
