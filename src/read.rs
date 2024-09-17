use std::io::{Read, Result, Seek};

pub trait Readable
where
    Self: Read + Seek,
{
    fn lock(&mut self) -> Result<()>;
    fn unlock(&mut self) -> Result<()>;
    fn close(self) -> Result<()>;

    fn size(&mut self) -> Result<u64> {
        self.seek(std::io::SeekFrom::End(0))
    }

    fn read_utf8_at(&mut self, pos: u64, len: u64) -> Result<String> {
        let mut buf = vec![0; len as usize];
        match self.seek(std::io::SeekFrom::Start(pos)) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match self.read_exact(&mut buf) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };
        match String::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}
