use std::io::Result;

use crate::{limit_rw, DataType, Readable, RwLimited, Source, Writable};

/// Provides methods to combine the [`Readable`] and [`Writable`] traits.
pub trait Rw<'a>
where
    Self: Readable<'a> + Writable<'a>,
{
    /// An internal method to get the reader as a trait object.
    /// Yes, this is kinda nonsense, but Rust forces me into that.
    ///
    /// ### How you implement it
    ///
    /// ```ignore
    /// fn rw_as_trait(&mut self) -> &mut dyn Rw<'a> {
    ///     self
    /// }
    /// ```
    fn rw_as_trait(&mut self) -> &mut dyn Rw<'a>;

    /// Borrows the read/write source.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut reader = dh::data::read(vec![0, 1, 2, 3, 4, 5, 6, 7]);
    /// let source = reader.source();
    /// match source {
    ///     Vec(source) => assert_eq!(source, &mut vec![0, 1, 2, 3, 4, 5, 6, 7]),
    ///     _ => unreachable!(),
    /// }
    /// ```
    fn rw_source(&mut self) -> Source;

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
    fn rw_limit(&'a mut self, start: u64, length: u64) -> Result<RwLimited<'a>> {
        limit_rw(self.rw_as_trait(), start, length)
    }
}
