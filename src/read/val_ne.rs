use super::{
    auto_impl::{auto_impl, auto_impl_nofns},
    val::read_dynamic_typed,
};
use crate::{Primitive, Result};
use std::io::Read;

macro_rules! read_primitive {
    ($fn_name:ident) => {
        /// Reads a primitive value from the reader using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `read_u8` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(&mut self) -> Result<T> {
            let mut buf = [0; S];
            self.read_exact(&mut buf)?;
            Ok(T::from_ne_bytes(buf))
        }
    };
}

macro_rules! read_primitive_typed {
    ($fn_name:ident, $return_type:ty, $_:ident) => {
        /// Typed wrapper around `read_ne`.
        fn $fn_name(&mut self) -> Result<$return_type> {
            self.read_ne()
        }
    };

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_ne`.
        fn $fn_name(&mut self) -> Result<$return_type> {
            self.read_ne()
        }
    };

    ($fn_name:ident, $return_type:ty, $_:ident, $const:ident) => {
        /// Typed wrapper around `read_ne`.
        fn $fn_name<const $const: usize>(&mut self) -> Result<$return_type> {
            self.read_ne::<$return_type, $return_type, $const>()
        }
    };
}

/// Extension trait for `Read` that provides methods for reading supported value types in native endian order.
///
/// **Note:** do not borrow this as `&mut dyn ReadValNe`, as this would not compile. Use `&mut dyn Read` instead.
pub trait ReadValNe: Read {
    auto_impl!();

    read_primitive!(read_ne);
}

impl<T: Read> ReadValNe for T {}
