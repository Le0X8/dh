use std::io::Result;

use crate::{DataType, Readable, Writable};

/// Provides methods to combine the [`Readable`] and [`Writable`] traits.
pub trait Rw<'a>
where
    Self: Readable<'a> + Writable<'a>,
{
    /// Closes the R/W stream and can return the target if it was moved or references it.
    ///
    /// Use this instead of [`Readable::close`] or [`Writable::close`] for cleaner code as you avoid the naming conflict.
    fn rw_close(self) -> Result<Option<DataType<'a>>>;

    /// Sets the stream position to the beginning.
    ///
    /// Use this instead of [`Readable::rewind`] or [`Writable::rewind`] for cleaner code as you avoid the naming conflict.
    fn rw_rewind(&mut self) -> Result<u64> {
        Writable::rewind(self)
    }

    /// Sets the stream position to the end. Reading or writing anything here can cause EOF errors under certain conditions.
    ///
    /// Use this instead of [`Readable::end`] or [`Writable::end`] for cleaner code as you avoid the naming conflict.
    fn rw_end(&mut self) -> Result<u64> {
        Writable::end(self)
    }

    /// Sets the stream position to a specific position.
    ///
    /// Use this instead of [`Readable::to`] or [`Writable::to`] for cleaner code as you avoid the naming conflict.
    fn rw_to(&mut self, pos: u64) -> Result<u64> {
        Writable::to(self, pos)
    }

    /// Jumps a specific amount of bytes from the current position.
    ///
    /// Use this instead of [`Readable::jump`] or [`Writable::jump`] for cleaner code as you avoid the naming conflict.
    fn rw_jump(&mut self, pos: i64) -> Result<u64> {
        Writable::jump(self, pos)
    }

    /// Returns the current stream position.
    ///
    /// Use this instead of [`Readable::size`] or [`Writable::size`] for cleaner code as you avoid the naming conflict.
    fn rw_size(&mut self) -> Result<u64> {
        Writable::size(self)
    }
}
