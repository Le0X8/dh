use crate::{Primitive, Result};
use std::io::Read;

macro_rules! read_primitive {
    ($fn_name:ident, $read_fn_name:ident) => {
        /// Reads a primitive value from the reader using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `read_u8` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(&mut self) -> Result<T> {
            let mut buf = [0; S];
            self.read_exact(&mut buf)?;
            Ok(T::$read_fn_name(buf))
        }
    };
}

macro_rules! read_primitive_typed {
    ($fn_name:ident, $return_type:ty, $read_fn:ident) => {
        /// Typed wrapper around `read_ne`, `read_le`, or `read_be`.
        fn $fn_name(&mut self) -> Result<$return_type> {
            self.$read_fn()
        }
    };

    ($fn_name:ident, $return_type:ty, $read_fn:ident, $const:ident) => {
        /// Typed wrapper around `read_ne`, `read_le`, or `read_be`.
        fn $fn_name<const $const: usize>(&mut self) -> Result<$return_type> {
            self.$read_fn::<$return_type, $return_type, $const>()
        }
    };
}

/// Extension trait for `Read` that provides methods for reading primitive values.
///
/// **Note:** do not borrow this as `&mut dyn ReadVal`, as this would not compile. Use `&mut dyn Read` instead.
pub trait ReadVal: Read {
    read_primitive!(read_ne, from_ne_bytes);
    read_primitive!(read_le, from_le_bytes);
    read_primitive!(read_be, from_be_bytes);

    read_primitive_typed!(read_u8, u8, read_ne);

    read_primitive_typed!(read_u16_ne, u16, read_ne);
    read_primitive_typed!(read_u16_le, u16, read_le);
    read_primitive_typed!(read_u16_be, u16, read_be);

    read_primitive_typed!(read_u32_ne, u32, read_ne);
    read_primitive_typed!(read_u32_le, u32, read_le);
    read_primitive_typed!(read_u32_be, u32, read_be);

    read_primitive_typed!(read_u64_ne, u64, read_ne);
    read_primitive_typed!(read_u64_le, u64, read_le);
    read_primitive_typed!(read_u64_be, u64, read_be);

    read_primitive_typed!(read_u128_ne, u128, read_ne);
    read_primitive_typed!(read_u128_le, u128, read_le);
    read_primitive_typed!(read_u128_be, u128, read_be);

    read_primitive_typed!(read_usize_ne, usize, read_ne);
    read_primitive_typed!(read_usize_le, usize, read_le);
    read_primitive_typed!(read_usize_be, usize, read_be);

    read_primitive_typed!(read_i8, i8, read_ne);

    read_primitive_typed!(read_i16_ne, i16, read_ne);
    read_primitive_typed!(read_i16_le, i16, read_le);
    read_primitive_typed!(read_i16_be, i16, read_be);

    read_primitive_typed!(read_i32_ne, i32, read_ne);
    read_primitive_typed!(read_i32_le, i32, read_le);
    read_primitive_typed!(read_i32_be, i32, read_be);

    read_primitive_typed!(read_i64_ne, i64, read_ne);
    read_primitive_typed!(read_i64_le, i64, read_le);
    read_primitive_typed!(read_i64_be, i64, read_be);

    read_primitive_typed!(read_i128_ne, i128, read_ne);
    read_primitive_typed!(read_i128_le, i128, read_le);
    read_primitive_typed!(read_i128_be, i128, read_be);

    read_primitive_typed!(read_isize_ne, isize, read_ne);
    read_primitive_typed!(read_isize_le, isize, read_le);
    read_primitive_typed!(read_isize_be, isize, read_be);

    read_primitive_typed!(read_f32_ne, f32, read_ne);
    read_primitive_typed!(read_f32_le, f32, read_le);
    read_primitive_typed!(read_f32_be, f32, read_be);

    read_primitive_typed!(read_f64_ne, f64, read_ne);
    read_primitive_typed!(read_f64_le, f64, read_le);
    read_primitive_typed!(read_f64_be, f64, read_be);

    read_primitive_typed!(read_u8_array, [u8; S], read_ne, S);

    read_primitive_typed!(read_bool, bool, read_ne);
}

impl<T: Read> ReadVal for T {}
