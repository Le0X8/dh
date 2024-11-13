use std::fs::File;

/// A helper enum to allow getting the source data from a [`Vec<u8>`] reader and/or writer.
///
/// **This data type is only intended for internal usage.**
pub enum DataType<'a> {
    Vec(Vec<u8>),
    Ref(&'a [u8]),
    Mut(&'a mut [u8]),
}

/// A helper enum to allow getting direct access on the data source.
///
/// **This data type is only intended for internal usage.**
pub enum Source<'a> {
    File(&'a mut File),
    Vec(&'a mut Vec<u8>),
    Ref(&'a [u8]),
    Mut(&'a mut [u8]),
}
