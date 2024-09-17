use crate::{Readable, Writable};

pub trait Rw
where
    Self: Readable + Writable,
{
}
