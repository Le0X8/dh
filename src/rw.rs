use std::io::Result;

use crate::{Readable, Writable};

pub trait Rw
where
    Self: Readable + Writable,
{
    fn rw_rewind(&mut self) -> Result<u64> {
        Writable::rewind(self)
    }

    fn rw_end(&mut self) -> Result<u64> {
        Writable::end(self)
    }

    fn rw_to(&mut self, pos: u64) -> Result<u64> {
        Writable::to(self, pos)
    }

    fn rw_jump(&mut self, pos: i64) -> Result<u64> {
        Writable::jump(self, pos)
    }

    fn rw_size(&mut self) -> Result<u64> {
        Writable::size(self)
    }
}
