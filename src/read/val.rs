#[cfg(feature = "vli")]
use crate::variable;
use crate::{Endianess, Primitive, Result, WriteSeek, WriteVal, WriteValAt};
use std::{
    cmp::min,
    io::{Read, Write},
};

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

    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_ne`, `read_le`, or `read_be`.
        fn $fn_name(&mut self, endianess: Endianess) -> Result<$return_type> {
            use Endianess::*;
            match endianess {
                Little => self.read_le(),
                Big => self.read_be(),
                Native => self.read_ne(),
            }
        }
    };

    ($fn_name:ident, $return_type:ty, $read_fn:ident, $const:ident) => {
        /// Typed wrapper around `read_ne`, `read_le`, or `read_be`.
        fn $fn_name<const $const: usize>(&mut self) -> Result<$return_type> {
            self.$read_fn::<$return_type, $return_type, $const>()
        }
    };
}

macro_rules! read_dynamic_typed {
    ($fn_name:ident, $return_type:ty) => {
        /// Typed wrapper around `read_dynamic`.
        fn $fn_name(&mut self, len: usize) -> Result<$return_type> {
            self.read_dynamic(len)
        }
    };
}

macro_rules! read_variable {
    ($fn_name:ident) => {
        /// Reads a variable-length integer from the reader.
        #[cfg(feature = "vli")]
        fn $fn_name(&mut self) -> Result<u128> {
            variable::$fn_name(self)
        }
    };
}

/// Extension trait for `Read` that provides methods for reading supported value types.
///
/// **Note:** do not borrow this as `&mut dyn ReadVal`, as this would not compile. Use `&mut dyn Read` instead.
pub trait ReadVal: Read {
    read_variable!(read_vu8);
    read_variable!(read_vu16_ne);
    read_variable!(read_vu16_le);
    read_variable!(read_vu16_be);
    read_variable!(read_vu32_ne);
    read_variable!(read_vu32_le);
    read_variable!(read_vu32_be);
    read_variable!(read_vu64_ne);
    read_variable!(read_vu64_le);
    read_variable!(read_vu64_be);
    read_variable!(read_vu128_ne);
    read_variable!(read_vu128_le);
    read_variable!(read_vu128_be);

    read_primitive_typed!(read_u8, u8, read_ne);

    read_primitive_typed!(read_u16, u16);
    read_primitive_typed!(read_u32, u32);
    read_primitive_typed!(read_u64, u64);
    read_primitive_typed!(read_u128, u128);
    read_primitive_typed!(read_usize, usize);

    read_primitive_typed!(read_i8, i8, read_ne);

    read_primitive_typed!(read_i16, i16);
    read_primitive_typed!(read_i32, i32);
    read_primitive_typed!(read_i64, i64);
    read_primitive_typed!(read_i128, i128);
    read_primitive_typed!(read_isize, isize);

    read_primitive_typed!(read_f32, f32);
    read_primitive_typed!(read_f64, f64);

    read_primitive_typed!(read_u8_array, [u8; S], read_ne, S);

    read_primitive_typed!(read_bool, bool, read_ne);

    read_dynamic_typed!(read_vec, Vec<u8>);
    read_dynamic_typed!(read_str, String);

    read_primitive!(read_ne, from_ne_bytes);
    read_primitive!(read_le, from_le_bytes);
    read_primitive!(read_be, from_be_bytes);

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

    /// Reads a dynamic value from the reader.
    ///
    /// It's recommended to use the typed wrappers like `read_vec` instead of this method for cleaner code.
    fn read_dynamic<T: crate::Dynamic>(&mut self, len: usize) -> Result<T> {
        let mut buf = vec![0; len];
        self.read_exact(&mut buf)?;
        T::from_bytes(buf)
    }

    fn copy(&mut self, len: usize, mut target: &mut dyn Write) -> Result<()> {
        target.write_vec(self.read_vec(len)?)
    }

    fn copy_to(
        &mut self,
        len: usize,
        targetpos: usize,
        mut target: &mut dyn WriteSeek,
    ) -> Result<()> {
        target.write_vec_at(targetpos, self.read_vec(len)?)
    }

    fn copy_chunked(
        &mut self,
        len: usize,
        mut target: &mut dyn Write,
        chunk_size: usize,
    ) -> Result<()> {
        let mut remaining = len;

        while remaining > 0 {
            let to_read = min(remaining, chunk_size);
            target.write_vec(self.read_vec(to_read)?)?;
            remaining -= to_read;
        }

        Ok(())
    }

    fn copy_chunked_to(
        &mut self,
        len: usize,
        targetpos: usize,
        mut target: &mut dyn WriteSeek,
        chunk_size: usize,
    ) -> Result<()> {
        let mut remaining = len;

        while remaining > 0 {
            let to_read = min(remaining, chunk_size);
            target.write_vec_at(targetpos + len - remaining, self.read_vec(to_read)?)?;
            remaining -= to_read;
        }

        Ok(())
    }
}

impl<T: Read> ReadVal for T {}
