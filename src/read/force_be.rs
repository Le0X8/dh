use super::auto_impl::auto_impl_nofns;
use crate::{Dynamic, Primitive};
use std::io::Read;

macro_rules! read_primitive {
    ($fn_name:ident) => {
        /// Reads a primitive value from the reader using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `read_u8` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(&mut self) -> T {
            let mut buf = [0; S];
            self.read_exact(&mut buf).unwrap();
            T::from_be_bytes(buf)
        }
    };
}

macro_rules! read_primitive_typed {
    ($fn_name:ident, $return_type:ty, $_:ident) => {
        /// Typed wrapper around `read_be`.
        fn $fn_name(&mut self) -> $return_type {
            self.read_be()
        }
    };

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_be`.
        fn $fn_name(&mut self) -> $return_type {
            self.read_be()
        }
    };

    ($fn_name:ident, $return_type:ty, $_:ident, $const:ident) => {
        /// Typed wrapper around `read_be`.
        fn $fn_name<const $const: usize>(&mut self) -> $return_type {
            self.read_be::<$return_type, $return_type, $const>()
        }
    };
}

macro_rules! read_dynamic_typed {
    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_dynamic`.
        fn $fn_name(&mut self, len: usize) -> $return_type {
            self.read_dynamic(len)
        }
    };
}

/// Extension trait for `Read` that provides methods for reading supported value types in big endian order without the `Result` return type, triggering a panic on error.
///
/// **Note:** do not borrow this as `&mut dyn ForceReadValBe`, as this would not compile. Use `&mut dyn Read` instead.
pub trait ForceReadValBe: Read {
    auto_impl_nofns!();

    read_primitive!(read_be);

    /// Reads a dynamic value from the reader.
    ///
    /// It's recommended to use the typed wrappers like `read_vec` instead of this method for cleaner code.
    fn read_dynamic<T: Dynamic>(&mut self, len: usize) -> T {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf).unwrap();
        T::from_bytes(buf).unwrap()
    }
}

impl<T: Read> ForceReadValBe for T {}
