macro_rules! auto_impl_nofns {
    () => {
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
    };
}

macro_rules! auto_impl_all {
    () => {
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
    };
}

macro_rules! auto_impl {
    () => {
        auto_impl_nofns!();

        /// Reads a dynamic value from the reader.
        ///
        /// It's recommended to use the typed wrappers like `read_vec` instead of this method for cleaner code.
        fn read_dynamic<T: crate::Dynamic>(&mut self, len: usize) -> Result<T> {
            let mut buf = vec![0; len];
            self.read_exact(&mut buf)?;
            T::from_bytes(buf)
        }
    };
}

pub(super) use auto_impl;
pub(super) use auto_impl_all;
pub(super) use auto_impl_nofns;
