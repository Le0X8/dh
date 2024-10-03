use std::io::Result;

use crate::{limit_rw, DataType, Readable, RwLimited, Writable};

/// Provides methods to combine the [`Readable`] and [`Writable`] traits.
pub trait Rw<'a>
where
    Self: Readable<'a> + Writable<'a>,
{
    /// Closes the R/W stream and can return the target if it was moved or references it.
    ///
    /// Use this instead of [`Readable::close`] or [`Writable::close`] for cleaner code as you avoid the naming conflict.
    fn rw_close(self) -> Result<Option<DataType<'a>>>;

    /// Limits the R/W stream to a certain range.
    ///
    /// Using [`Readable::limit`] and [`Writable::limit`] would not
    /// return [`RwLimited`] and would be limited to either reading or writing.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut data = vec![0, 1, 2, 3];
    /// let mut rw = dh::data::rw_ref(&mut data);
    ///
    /// let mut limited = rw.rw_limit(1, 2).unwrap();
    /// limited.to(1).unwrap();
    /// limited.write_u8(4).unwrap();
    ///
    /// limited.rewind().unwrap();
    /// assert_eq!(limited.read_u16be().unwrap(), 0x0104);
    /// ```
    fn rw_limit(&'a mut self, start: u64, length: u64) -> Result<RwLimited<'a, Self>> {
        limit_rw(self, start, length)
    }
}
