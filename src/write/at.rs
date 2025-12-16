use crate::{Endianess, Primitive, Result};
use std::io::{Seek, SeekFrom::Start as SeekPos, Write};

macro_rules! write_primitive {
    ($fn_name:ident, $write_fn_name:ident) => {
        /// Writes a primitive value to the writer using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `write_u8_at` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(
            &mut self,
            pos: usize,
            data: T,
        ) -> Result<()> {
            let pos_before = self.stream_position()?;
            self.seek(SeekPos(pos as u64))?;

            let response = self.write_all(&data.$write_fn_name());

            self.seek(SeekPos(pos_before))?;
            response
        }
    };
}

macro_rules! write_primitive_typed {
    ($fn_name:ident, $return_type:ty, $write_fn:ident) => {
        /// Typed wrapper around `write_ne_at`, `write_le_at`, or `write_be_at`.
        fn $fn_name(&mut self, pos: usize, data: $return_type) -> Result<()> {
            self.$write_fn(pos, data)
        }
    };

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `write_ne_at`, `write_le_at`, or `write_be_at`.
        fn $fn_name(&mut self, pos: usize, endianess: Endianess, data: $return_type) -> Result<()> {
            use Endianess::*;
            match endianess {
                Little => self.write_le_at(pos, data),
                Big => self.write_be_at(pos, data),
                Native => self.write_ne_at(pos, data),
            }
        }
    };

    ($fn_name:ident, $return_type:ty, $write_fn:ident, $const:ident) => {
        /// Typed wrapper around `write_ne_at`, `write_le_at`, or `write_be_at`.
        fn $fn_name<const $const: usize>(&mut self, pos: usize, data: $return_type) -> Result<()> {
            self.$write_fn::<$return_type, $return_type, $const>(pos, data)
        }
    };
}

macro_rules! write_dynamic_typed {
    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `write_dynamic_at`.
        fn $fn_name(&mut self, pos: usize, data: $return_type) -> Result<()> {
            let pos_before = self.stream_position()?;
            self.seek(SeekPos(pos as u64))?;
            let response = self.write_dynamic_at(data);
            self.seek(SeekPos(pos_before))?;
            response
        }
    };
}

/// Extension trait for `Write + Seek` that provides methods for writeing supported value types.
///
/// **Note:** do not borrow this as `&mut dyn WriteValAt`, as this would not compile. Use `&mut dyn dh::WriteSeek` instead.
pub trait WriteValAt: Write + Seek {
    write_primitive_typed!(write_u8_at, u8, write_ne_at);

    write_primitive_typed!(write_u16_at, u16);
    write_primitive_typed!(write_u32_at, u32);
    write_primitive_typed!(write_u64_at, u64);
    write_primitive_typed!(write_u128_at, u128);
    write_primitive_typed!(write_usize_at, usize);

    write_primitive_typed!(write_i8_at, i8, write_ne_at);

    write_primitive_typed!(write_i16_at, i16);
    write_primitive_typed!(write_i32_at, i32);
    write_primitive_typed!(write_i64_at, i64);
    write_primitive_typed!(write_i128_at, i128);
    write_primitive_typed!(write_isize_at, isize);

    write_primitive_typed!(write_f32_at, f32);
    write_primitive_typed!(write_f64_at, f64);

    write_primitive_typed!(write_u8_array_at, [u8; S], write_ne_at, S);

    write_primitive_typed!(write_bool_at, bool, write_ne_at);

    write_dynamic_typed!(write_vec_at, Vec<u8>);
    write_dynamic_typed!(write_str_at, String);

    write_primitive!(write_ne_at, to_ne_bytes);
    write_primitive!(write_le_at, to_le_bytes);
    write_primitive!(write_be_at, to_be_bytes);

    write_primitive_typed!(write_u16_ne_at, u16, write_ne_at);
    write_primitive_typed!(write_u16_le_at, u16, write_le_at);
    write_primitive_typed!(write_u16_be_at, u16, write_be_at);

    write_primitive_typed!(write_u32_ne_at, u32, write_ne_at);
    write_primitive_typed!(write_u32_le_at, u32, write_le_at);
    write_primitive_typed!(write_u32_be_at, u32, write_be_at);

    write_primitive_typed!(write_u64_ne_at, u64, write_ne_at);
    write_primitive_typed!(write_u64_le_at, u64, write_le_at);
    write_primitive_typed!(write_u64_be_at, u64, write_be_at);

    write_primitive_typed!(write_u128_ne_at, u128, write_ne_at);
    write_primitive_typed!(write_u128_le_at, u128, write_le_at);
    write_primitive_typed!(write_u128_be_at, u128, write_be_at);

    write_primitive_typed!(write_usize_ne_at, usize, write_ne_at);
    write_primitive_typed!(write_usize_le_at, usize, write_le_at);
    write_primitive_typed!(write_usize_be_at, usize, write_be_at);

    write_primitive_typed!(write_i16_ne_at, i16, write_ne_at);
    write_primitive_typed!(write_i16_le_at, i16, write_le_at);
    write_primitive_typed!(write_i16_be_at, i16, write_be_at);

    write_primitive_typed!(write_i32_ne_at, i32, write_ne_at);
    write_primitive_typed!(write_i32_le_at, i32, write_le_at);
    write_primitive_typed!(write_i32_be_at, i32, write_be_at);

    write_primitive_typed!(write_i64_ne_at, i64, write_ne_at);
    write_primitive_typed!(write_i64_le_at, i64, write_le_at);
    write_primitive_typed!(write_i64_be_at, i64, write_be_at);

    write_primitive_typed!(write_i128_ne_at, i128, write_ne_at);
    write_primitive_typed!(write_i128_le_at, i128, write_le_at);
    write_primitive_typed!(write_i128_be_at, i128, write_be_at);

    write_primitive_typed!(write_isize_ne_at, isize, write_ne_at);
    write_primitive_typed!(write_isize_le_at, isize, write_le_at);
    write_primitive_typed!(write_isize_be_at, isize, write_be_at);

    write_primitive_typed!(write_f32_ne_at, f32, write_ne_at);
    write_primitive_typed!(write_f32_le_at, f32, write_le_at);
    write_primitive_typed!(write_f32_be_at, f32, write_be_at);

    write_primitive_typed!(write_f64_ne_at, f64, write_ne_at);
    write_primitive_typed!(write_f64_le_at, f64, write_le_at);
    write_primitive_typed!(write_f64_be_at, f64, write_be_at);

    /// Writes a dynamic value to the writeer.
    ///
    /// It's recommended to use the typed wrappers like `write_vec_at` instead of this method for cleaner code.
    fn write_dynamic_at<T: crate::Dynamic>(&mut self, data: T) -> Result<()> {
        self.write_all(data.into_bytes()?)
    }
}

impl<T: Write + Seek> WriteValAt for T {}
