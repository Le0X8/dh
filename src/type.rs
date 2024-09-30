pub enum DataType<'a> {
    Vec(Vec<u8>),
    VecRef(&'a Vec<u8>),
}
