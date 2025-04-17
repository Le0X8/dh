use crate::error::Result;
use std::io::{Error, ErrorKind::InvalidData};

// marker trait
pub trait Dynamic: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>;
}

impl Dynamic for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(bytes)
    }
}

impl Dynamic for String {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        String::from_utf8(bytes).map_err(|_| Error::new(InvalidData, "Invalid UTF-8"))
    }
}
