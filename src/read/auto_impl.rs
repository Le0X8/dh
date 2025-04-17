macro_rules! auto_impl {
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

        /// Reads a dynamic value from the reader.
        ///
        /// It's recommended to use the typed wrappers like `read_vec` instead of this method for cleaner code.
        fn read_dynamic<T: crate::Dynamic>(&mut self, len: usize) -> Result<T> {
            let mut buf = vec![0; len];
            self.read_exact(&mut buf)?;
            T::from_bytes(buf)
        }

        read_dynamic_typed!(read_vec, Vec<u8>);
        read_dynamic_typed!(read_str, String);
    };
}

pub(super) use auto_impl;
