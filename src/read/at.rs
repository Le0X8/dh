use crate::{Endianess, Primitive, Result};
use std::io::{Read, Seek, SeekFrom::Start as SeekPos};

macro_rules! read_primitive {
    ($fn_name:ident, $read_fn_name:ident) => {
        /// Reads a primitive value from the reader using the specified byte order.
        ///
        /// It's recommended to use the typed wrappers like `read_u8_at` instead of this method for cleaner code.
        fn $fn_name<T: Primitive<U, S>, U, const S: usize>(&mut self, pos: usize) -> Result<T> {
            let mut buf = [0; S];
            let pos_before = self.stream_position()?;
            self.seek(SeekPos(pos as u64))?;
            self.read_exact(&mut buf)?;
            self.seek(SeekPos(pos_before))?;
            Ok(T::$read_fn_name(buf))
        }
    };
}

macro_rules! read_primitive_typed {
    ($fn_name:ident, $return_type:ty, $read_fn:ident) => {
        /// Typed wrapper around `read_ne_at`, `read_le_at`, or `read_be_at`.
        fn $fn_name(&mut self, pos: usize) -> Result<$return_type> {
            self.$read_fn(pos)
        }
    };

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_ne_at`, `read_le_at`, or `read_be_at`.
        fn $fn_name(&mut self, pos: usize, endianess: Endianess) -> Result<$return_type> {
            use Endianess::*;
            match endianess {
                Little => self.read_le_at(pos),
                Big => self.read_be_at(pos),
                Native => self.read_ne_at(pos),
            }
        }
    };

    ($fn_name:ident, $return_type:ty, $read_fn:ident, $const:ident) => {
        /// Typed wrapper around `read_ne_at`, `read_le_at`, or `read_be_at`.
        fn $fn_name<const $const: usize>(&mut self, pos: usize) -> Result<$return_type> {
            self.$read_fn::<$return_type, $return_type, $const>(pos)
        }
    };
}

macro_rules! read_dynamic_typed {
    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_dynamic_at`.
        fn $fn_name(&mut self, pos: usize, len: usize) -> Result<$return_type> {
            let pos_before = self.stream_position()?;
            self.seek(SeekPos(pos as u64))?;
            let result = self.read_dynamic(len);
            self.seek(SeekPos(pos_before))?;
            result
        }
    };
}

pub(super) use read_dynamic_typed;
pub(super) use read_primitive;
pub(super) use read_primitive_typed;

/// Extension trait for `Read` that provides methods for reading supported value types.
///
/// **Note:** do not borrow this as `&mut dyn ReadValAt`, as this would not compile. Use `&mut dyn dh::ReadSeek` instead.
pub trait ReadValAt: Read + Seek {
    read_primitive_typed!(read_u8_at, u8, read_ne_at);

    read_primitive_typed!(read_u16_at, u16);
    read_primitive_typed!(read_u32_at, u32);
    read_primitive_typed!(read_u64_at, u64);
    read_primitive_typed!(read_u128_at, u128);
    read_primitive_typed!(read_usize_at, usize);

    read_primitive_typed!(read_i8_at, i8, read_ne_at);

    read_primitive_typed!(read_i16_at, i16);
    read_primitive_typed!(read_i32_at, i32);
    read_primitive_typed!(read_i64_at, i64);
    read_primitive_typed!(read_i128_at, i128);
    read_primitive_typed!(read_isize_at, isize);

    read_primitive_typed!(read_f32_at, f32);
    read_primitive_typed!(read_f64_at, f64);

    read_primitive_typed!(read_u8_array_at, [u8; S], read_ne_at, S);

    read_primitive_typed!(read_bool_at, bool, read_ne_at);

    read_dynamic_typed!(read_vec_at, Vec<u8>);
    read_dynamic_typed!(read_str_at, String);

    read_primitive!(read_ne_at, from_ne_bytes);
    read_primitive!(read_le_at, from_le_bytes);
    read_primitive!(read_be_at, from_be_bytes);

    read_primitive_typed!(read_u16_ne_at, u16, read_ne_at);
    read_primitive_typed!(read_u16_le_at, u16, read_le_at);
    read_primitive_typed!(read_u16_be_at, u16, read_be_at);

    read_primitive_typed!(read_u32_ne_at, u32, read_ne_at);
    read_primitive_typed!(read_u32_le_at, u32, read_le_at);
    read_primitive_typed!(read_u32_be_at, u32, read_be_at);

    read_primitive_typed!(read_u64_ne_at, u64, read_ne_at);
    read_primitive_typed!(read_u64_le_at, u64, read_le_at);
    read_primitive_typed!(read_u64_be_at, u64, read_be_at);

    read_primitive_typed!(read_u128_ne_at, u128, read_ne_at);
    read_primitive_typed!(read_u128_le_at, u128, read_le_at);
    read_primitive_typed!(read_u128_be_at, u128, read_be_at);

    read_primitive_typed!(read_usize_ne_at, usize, read_ne_at);
    read_primitive_typed!(read_usize_le_at, usize, read_le_at);
    read_primitive_typed!(read_usize_be_at, usize, read_be_at);

    read_primitive_typed!(read_i16_ne_at, i16, read_ne_at);
    read_primitive_typed!(read_i16_le_at, i16, read_le_at);
    read_primitive_typed!(read_i16_be_at, i16, read_be_at);

    read_primitive_typed!(read_i32_ne_at, i32, read_ne_at);
    read_primitive_typed!(read_i32_le_at, i32, read_le_at);
    read_primitive_typed!(read_i32_be_at, i32, read_be_at);

    read_primitive_typed!(read_i64_ne_at, i64, read_ne_at);
    read_primitive_typed!(read_i64_le_at, i64, read_le_at);
    read_primitive_typed!(read_i64_be_at, i64, read_be_at);

    read_primitive_typed!(read_i128_ne_at, i128, read_ne_at);
    read_primitive_typed!(read_i128_le_at, i128, read_le_at);
    read_primitive_typed!(read_i128_be_at, i128, read_be_at);

    read_primitive_typed!(read_isize_ne_at, isize, read_ne_at);
    read_primitive_typed!(read_isize_le_at, isize, read_le_at);
    read_primitive_typed!(read_isize_be_at, isize, read_be_at);

    read_primitive_typed!(read_f32_ne_at, f32, read_ne_at);
    read_primitive_typed!(read_f32_le_at, f32, read_le_at);
    read_primitive_typed!(read_f32_be_at, f32, read_be_at);

    read_primitive_typed!(read_f64_ne_at, f64, read_ne_at);
    read_primitive_typed!(read_f64_le_at, f64, read_le_at);
    read_primitive_typed!(read_f64_be_at, f64, read_be_at);

    /// Reads a dynamic value from the reader.
    ///
    /// It's recommended to use the typed wrappers like `read_vec` instead of this method for cleaner code.
    fn read_dynamic<T: crate::Dynamic>(&mut self, len: usize) -> Result<T> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        T::from_bytes(buf)
    }
}

impl<T: Read + Seek> ReadValAt for T {}
