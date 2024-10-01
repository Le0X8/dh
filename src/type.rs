/// A helper enum to allow getting the source data from a [`Vec<u8>`] reader and/or writer.
///
/// **This data type is only intended for internal usage.**
pub enum DataType<'a> {
    Vec(Vec<u8>),
    VecRef(&'a Vec<u8>),
    VecMut(&'a mut Vec<u8>),
}
