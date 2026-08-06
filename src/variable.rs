use crate::{ReadVal, WriteVal, internal::Result};
use std::io::{Read, Write};

macro_rules! rw_unsigned {
    ($read_fn_name:ident, $read_fn:ident, $write_fn_name:ident, $write_fn:ident, $type:ty) => {
        pub fn $read_fn_name<T: Read + ?Sized>(reader: &mut T) -> Result<u128> {
            const BITS: u32 = size_of::<$type>() as u32 * 8 - 1;
            const HIGHEST: u128 = 1 << BITS;
            const LOWER: u128 = HIGHEST - 1;

            let mut value = 0;
            let mut shift = 0;
            loop {
                let chunk = reader.$read_fn()? as u128;
                value |= (chunk & LOWER).checked_shl(shift).ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Variable-length integer is too large",
                    )
                })?;
                shift += BITS;
                if chunk & HIGHEST == 0 {
                    break;
                }
            }
            Ok(value)
        }

        pub fn $write_fn_name<T: Write + ?Sized>(writer: &mut T, value: u128) -> Result<()> {
            const BITS: u32 = size_of::<$type>() as u32 * 8 - 1;
            const HIGHEST: u128 = 1 << BITS;
            const LOWER: u128 = HIGHEST - 1;

            let mut value = value;

            loop {
                let not_last = value > LOWER;
                let chunk = (value & LOWER) | if not_last { HIGHEST } else { 0 };
                writer.$write_fn(chunk as _)?;
                if !not_last {
                    break;
                }
                value >>= BITS;
            }

            Ok(())
        }
    };
}

rw_unsigned!(read_vu8, read_u8, write_vu8, write_u8, u8);
rw_unsigned!(read_vu16_ne, read_u16_ne, write_vu16_ne, write_u16_ne, u16);
rw_unsigned!(read_vu16_le, read_u16_le, write_vu16_le, write_u16_le, u16);
rw_unsigned!(read_vu16_be, read_u16_be, write_vu16_be, write_u16_be, u16);
rw_unsigned!(read_vu32_ne, read_u32_ne, write_vu32_ne, write_u32_ne, u32);
rw_unsigned!(read_vu32_le, read_u32_le, write_vu32_le, write_u32_le, u32);
rw_unsigned!(read_vu32_be, read_u32_be, write_vu32_be, write_u32_be, u32);
rw_unsigned!(read_vu64_ne, read_u64_ne, write_vu64_ne, write_u64_ne, u64);
rw_unsigned!(read_vu64_le, read_u64_le, write_vu64_le, write_u64_le, u64);
rw_unsigned!(read_vu64_be, read_u64_be, write_vu64_be, write_u64_be, u64);
rw_unsigned!(
    read_vu128_ne,
    read_u128_ne,
    write_vu128_ne,
    write_u128_ne,
    u128
);
rw_unsigned!(
    read_vu128_le,
    read_u128_le,
    write_vu128_le,
    write_u128_le,
    u128
);
rw_unsigned!(
    read_vu128_be,
    read_u128_be,
    write_vu128_be,
    write_u128_be,
    u128
);

// TODO: Variable-length signed integers
