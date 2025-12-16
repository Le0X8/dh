#[cfg(feature = "vli")]
use crate::variable;
use crate::{Endianess, Primitive, Result};
use std::io::Write;

macro_rules! write_primitive {
    ($fn_name:ident, $write_fn_name:ident) => {
        /// Writes a primitive value to the writer using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `write_u8` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(&mut self, data: T) -> Result<()> {
            self.write_all(&data.$write_fn_name())
        }
    };
}

macro_rules! write_primitive_typed {
    ($fn_name:ident, $return_type:ty, $write_fn:ident) => {
        /// Typed wrapper around `write_ne`, `write_le`, or `write_be`.
        fn $fn_name(&mut self, data: $return_type) -> Result<()> {
            self.$write_fn(data)
        }
    };

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `write_ne`, `write_le`, or `write_be`.
        fn $fn_name(&mut self, endianess: Endianess, data: $return_type) -> Result<()> {
            use Endianess::*;
            match endianess {
                Little => self.write_le(data),
                Big => self.write_be(data),
                Native => self.write_ne(data),
            }
        }
    };

    ($fn_name:ident, $return_type:ty, $write_fn:ident, $const:ident) => {
        /// Typed wrapper around `write_ne`, `write_le`, or `write_be`.
        fn $fn_name<const $const: usize>(&mut self, data: $return_type) -> Result<()> {
            self.$write_fn::<$return_type, $return_type, $const>(data)
        }
    };
}

macro_rules! write_dynamic_typed {
    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `write_dynamic`.
        fn $fn_name(&mut self, data: $return_type) -> Result<()> {
            self.write_dynamic(data)
        }
    };
}

macro_rules! write_variable {
    ($fn_name:ident) => {
        /// Writes a variable-length integer to the reader.
        #[cfg(feature = "vli")]
        fn $fn_name(&mut self, value: u128) -> Result<()> {
            variable::$fn_name(self, value)
        }
    };
}

/// Extension trait for `Write` that provides methods for writeing supported value types.
///
/// **Note:** do not borrow this as `&mut dyn WriteVal`, as this would not compile. Use `&mut dyn Write` instead.
pub trait WriteVal: Write {
    write_variable!(write_vu8);
    write_variable!(write_vu16_ne);
    write_variable!(write_vu16_le);
    write_variable!(write_vu16_be);
    write_variable!(write_vu32_ne);
    write_variable!(write_vu32_le);
    write_variable!(write_vu32_be);
    write_variable!(write_vu64_ne);
    write_variable!(write_vu64_le);
    write_variable!(write_vu64_be);
    write_variable!(write_vu128_ne);
    write_variable!(write_vu128_le);
    write_variable!(write_vu128_be);

    write_primitive_typed!(write_u8, u8, write_ne);

    write_primitive_typed!(write_u16, u16);
    write_primitive_typed!(write_u32, u32);
    write_primitive_typed!(write_u64, u64);
    write_primitive_typed!(write_u128, u128);
    write_primitive_typed!(write_usize, usize);

    write_primitive_typed!(write_i8, i8, write_ne);

    write_primitive_typed!(write_i16, i16);
    write_primitive_typed!(write_i32, i32);
    write_primitive_typed!(write_i64, i64);
    write_primitive_typed!(write_i128, i128);
    write_primitive_typed!(write_isize, isize);

    write_primitive_typed!(write_f32, f32);
    write_primitive_typed!(write_f64, f64);

    write_primitive_typed!(write_u8_array, [u8; S], write_ne, S);

    write_primitive_typed!(write_bool, bool, write_ne);

    write_dynamic_typed!(write_vec, Vec<u8>);
    write_dynamic_typed!(write_str, String);

    write_primitive!(write_ne, to_ne_bytes);
    write_primitive!(write_le, to_le_bytes);
    write_primitive!(write_be, to_be_bytes);

    write_primitive_typed!(write_u16_ne, u16, write_ne);
    write_primitive_typed!(write_u16_le, u16, write_le);
    write_primitive_typed!(write_u16_be, u16, write_be);

    write_primitive_typed!(write_u32_ne, u32, write_ne);
    write_primitive_typed!(write_u32_le, u32, write_le);
    write_primitive_typed!(write_u32_be, u32, write_be);

    write_primitive_typed!(write_u64_ne, u64, write_ne);
    write_primitive_typed!(write_u64_le, u64, write_le);
    write_primitive_typed!(write_u64_be, u64, write_be);

    write_primitive_typed!(write_u128_ne, u128, write_ne);
    write_primitive_typed!(write_u128_le, u128, write_le);
    write_primitive_typed!(write_u128_be, u128, write_be);

    write_primitive_typed!(write_usize_ne, usize, write_ne);
    write_primitive_typed!(write_usize_le, usize, write_le);
    write_primitive_typed!(write_usize_be, usize, write_be);

    write_primitive_typed!(write_i16_ne, i16, write_ne);
    write_primitive_typed!(write_i16_le, i16, write_le);
    write_primitive_typed!(write_i16_be, i16, write_be);

    write_primitive_typed!(write_i32_ne, i32, write_ne);
    write_primitive_typed!(write_i32_le, i32, write_le);
    write_primitive_typed!(write_i32_be, i32, write_be);

    write_primitive_typed!(write_i64_ne, i64, write_ne);
    write_primitive_typed!(write_i64_le, i64, write_le);
    write_primitive_typed!(write_i64_be, i64, write_be);

    write_primitive_typed!(write_i128_ne, i128, write_ne);
    write_primitive_typed!(write_i128_le, i128, write_le);
    write_primitive_typed!(write_i128_be, i128, write_be);

    write_primitive_typed!(write_isize_ne, isize, write_ne);
    write_primitive_typed!(write_isize_le, isize, write_le);
    write_primitive_typed!(write_isize_be, isize, write_be);

    write_primitive_typed!(write_f32_ne, f32, write_ne);
    write_primitive_typed!(write_f32_le, f32, write_le);
    write_primitive_typed!(write_f32_be, f32, write_be);

    write_primitive_typed!(write_f64_ne, f64, write_ne);
    write_primitive_typed!(write_f64_le, f64, write_le);
    write_primitive_typed!(write_f64_be, f64, write_be);

    /// Writes a dynamic value to the writeer.
    ///
    /// It's recommended to use the typed wrappers like `write_vec` instead of this method for cleaner code.
    fn write_dynamic<T: crate::Dynamic>(&mut self, data: T) -> Result<()> {
        self.write_all(data.into_bytes()?)
    }
}

impl<T: Write> WriteVal for T {}
