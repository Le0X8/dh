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
}
