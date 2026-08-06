use std::io::{Read, Seek, Write};

/// Helper trait for referencing [`Read`] and [`Seek`].
pub trait Rs {}
/// Helper trait for referencing [`Write`] and [`Seek`].
pub trait Ws {}
/// Helper trait for referencing [`Read`] and [`Write`].
pub trait Rw {}
/// Helper trait for referencing [`Read`], [`Write`], and [`Seek`].
pub trait Rws {}

impl<T: Read + Seek + ?Sized> Rs for T {}
impl<T: Write + Seek + ?Sized> Ws for T {}
impl<T: Read + Write + ?Sized> Rw for T {}
impl<T: Read + Write + Seek + ?Sized> Rws for T {}
