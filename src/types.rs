use std::io::{Read, Seek};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianess {
    Little,
    Big,
    Native,
}

/// Because you can't do `dyn Read + Seek` in Rust, this trait is used to combine both traits.
pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}
