use std::io::{Result, Seek, SeekFrom};

/// Provides methods to seek a stream.
///
/// This trait is automatically implemented for any type that implements [`Seek`], that
/// includes any type that implements [`Readable`][crate::Readable],
/// [`Writable`][crate::Writable] and [`Rw`][crate::Rw] as all these traits require [`Seek`] to be implemented.
pub trait Seekable: Seek {
    /// Sets the stream position to the beginning.
    /// This is the same as calling [`to`][Seekable::to] with `0`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_u8(0x80).unwrap(); // reads the first byte
    /// assert_eq!(writer.pos().unwrap(), 1);
    ///
    /// writer.rewind().unwrap(); // sets the position to the beginning
    /// assert_eq!(writer.pos().unwrap(), 0);
    /// ```
    fn rewind(&mut self) -> Result<u64> {
        self.seek(SeekFrom::Start(0))
    }

    /// Sets the stream position to the end. It is not recommended to read anything after this because it would result in an EOF error.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.end().unwrap(); // sets the position to the end
    /// assert_eq!(writer.pos().unwrap(), 2);
    /// ```
    fn end(&mut self) -> Result<u64> {
        self.seek(SeekFrom::End(0))
    }

    /// Sets the stream position to a specific position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.to(1).unwrap(); // sets the position to the second byte
    /// assert_eq!(writer.pos().unwrap(), 1);
    /// ```
    fn to(&mut self, pos: u64) -> Result<u64> {
        self.seek(SeekFrom::Start(pos))
    }

    /// Jumps a specific amount of bytes from the current position.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.to(1).unwrap(); // sets the position to the second byte
    ///
    /// writer.jump(1).unwrap(); // sets the position to the next byte
    /// assert_eq!(writer.pos().unwrap(), 2);
    ///
    /// writer.jump(-1).unwrap(); // sets the position to the previous byte
    /// assert_eq!(writer.pos().unwrap(), 1);
    /// ```
    fn jump(&mut self, pos: i64) -> Result<u64> {
        self.seek(SeekFrom::Current(pos))
    }

    /// Calculates the current size of the source.
    /// Please call this method as less as possible because it moves to the end of the stream and back to the previous position.
    /// If you need to know the size of the source, consider storing it in a variable.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// assert_eq!(writer.size().unwrap(), 2);
    /// ```
    fn size(&mut self) -> Result<u64> {
        let pos_before = self.stream_position()?;
        let size = self.end();
        self.to(pos_before)?;
        size
    }

    /// Returns the current position of the stream.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use dh::recommended::*;
    ///
    /// let mut writer = dh::data::write_new(2);
    ///
    /// writer.write_u8(0x80).unwrap(); // reads the first byte
    /// assert_eq!(writer.pos().unwrap(), 1);
    /// ```
    fn pos(&mut self) -> Result<u64> {
        self.stream_position()
    }
}
