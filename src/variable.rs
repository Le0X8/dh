use crate::{ReadVal, Result, WriteVal};
use std::io::{Read, Write};

macro_rules! rw_unsigned {
    ($read_fn_name:ident, $read_fn:ident, $write_fn_name:ident, $write_fn:ident, $bits:literal, $lower_bits:literal, $highest_bit:literal) => {
        pub fn $read_fn_name<T: Read + ?Sized>(mut reader: &mut T) -> Result<u128> {
            let mut value = 0;
            loop {
                let chunk = reader.$read_fn()?;
                value = (value << $bits) | (chunk as u128 & $lower_bits);
                if chunk & $highest_bit == 0 {
                    break;
                }
            }
            Ok(value)
        }

        pub fn $write_fn_name<T: Write + ?Sized>(mut writer: &mut T, value: u128) -> Result<()> {
            let mut value = value;

            loop {
                let not_last = value > $lower_bits;
                let chunk = (value & $lower_bits) | if not_last { $highest_bit } else { 0 };
                writer.$write_fn(chunk as _)?;
                if !not_last {
                    break;
                }
                value >>= $bits;
            }

            Ok(())
        }
    };
}

rw_unsigned!(read_vu8, read_u8, write_vu8, write_u8, 7, 0x7f, 0x80);
rw_unsigned!(
    read_vu16_ne,
    read_u16_ne,
    write_vu16_ne,
    write_u16_ne,
    15,
    0x7fff,
    0x8000
);
rw_unsigned!(
    read_vu16_le,
    read_u16_le,
    write_vu16_le,
    write_u16_le,
    15,
    0x7fff,
    0x8000
);
rw_unsigned!(
    read_vu16_be,
    read_u16_be,
    write_vu16_be,
    write_u16_be,
    15,
    0x7fff,
    0x8000
);
rw_unsigned!(
    read_vu32_ne,
    read_u32_ne,
    write_vu32_ne,
    write_u32_ne,
    31,
    0x7fffffff,
    0x80000000
);
rw_unsigned!(
    read_vu32_le,
    read_u32_le,
    write_vu32_le,
    write_u32_le,
    31,
    0x7fffffff,
    0x80000000
);
rw_unsigned!(
    read_vu32_be,
    read_u32_be,
    write_vu32_be,
    write_u32_be,
    31,
    0x7fffffff,
    0x80000000
);
rw_unsigned!(
    read_vu64_ne,
    read_u64_ne,
    write_vu64_ne,
    write_u64_ne,
    63,
    0x7fffffffffffffff,
    0x8000000000000000
);
rw_unsigned!(
    read_vu64_le,
    read_u64_le,
    write_vu64_le,
    write_u64_le,
    63,
    0x7fffffffffffffff,
    0x8000000000000000
);
rw_unsigned!(
    read_vu64_be,
    read_u64_be,
    write_vu64_be,
    write_u64_be,
    63,
    0x7fffffffffffffff,
    0x8000000000000000
);
rw_unsigned!(
    read_vu128_ne,
    read_u128_ne,
    write_vu128_ne,
    write_u128_ne,
    127,
    0x7fffffffffffffffffffffffffffffff,
    0x80000000000000000000000000000000
);
rw_unsigned!(
    read_vu128_le,
    read_u128_le,
    write_vu128_le,
    write_u128_le,
    127,
    0x7fffffffffffffffffffffffffffffff,
    0x80000000000000000000000000000000
);
rw_unsigned!(
    read_vu128_be,
    read_u128_be,
    write_vu128_be,
    write_u128_be,
    127,
    0x7fffffffffffffffffffffffffffffff,
    0x80000000000000000000000000000000
);

// TODO: Variable-length signed integers
