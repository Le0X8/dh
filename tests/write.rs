use dh::{Endianess::Little, WriteVal};
use std::io::{Cursor, Write};

#[test]
fn write_borrowing() {
    let mut cursor = Cursor::new([0u8; 4]);

    // this just needs to compile
    let mut borrowed: &mut dyn Write = &mut cursor;

    borrowed.write_u8(0).unwrap();
    borrowed.write_u8(1).unwrap();
    borrowed.write_u8(2).unwrap();
    borrowed.write_u8(3).unwrap();

    assert_eq!(cursor.get_ref(), &[0, 1, 2, 3]);
}

#[test]
fn write_primitive() {
    let mut cursor = Cursor::new([0u8; 8]);

    cursor.write_ne(0u8).unwrap();
    cursor.write_ne([1u8, 2, 3]).unwrap();
    cursor.write_be(0x04_05u16).unwrap();
    cursor.write_le(0x07_06u16).unwrap();

    assert_eq!(cursor.get_ref(), &[0, 1, 2, 3, 4, 5, 6, 7]);

    // overflow
    let val = cursor.write_ne(0u8);
    assert!(val.is_err());
}

#[test]
fn write_u8() {
    let mut cursor = Cursor::new([0u8; 4]);

    cursor.write_u8(0).unwrap();
    cursor.write_u8(1).unwrap();
    cursor.write_u8(2).unwrap();
    cursor.write_u8(3).unwrap();

    assert_eq!(cursor.get_ref(), &[0, 1, 2, 3]);

    // overflow
    let val = cursor.write_u8(0);
    assert!(val.is_err());
}

#[test]
fn write_u16_ne() {
    let mut cursor = Cursor::new([0u8; 4]);

    let parts = match cfg!(target_endian = "big") {
        true => (0x1234, 0x5678),
        false => (0x3412, 0x7856),
    };

    cursor.write_u16_ne(parts.0).unwrap();
    cursor.write_u16_ne(parts.1).unwrap();

    assert_eq!(cursor.get_ref(), &[0x12, 0x34, 0x56, 0x78]);

    // overflow
    let val = cursor.write_u16_ne(0);
    assert!(val.is_err());
}

#[test]
fn write_u16_le() {
    let mut cursor = Cursor::new([0u8; 4]);

    cursor.write_u16(Little, 0x3412).unwrap();
    cursor.write_u16(Little, 0x7856).unwrap();

    assert_eq!(cursor.get_ref(), &[0x12, 0x34, 0x56, 0x78]);

    // overflow
    let val = cursor.write_u16_le(0);
    assert!(val.is_err());
}

#[test]
fn write_u16_be() {
    let mut cursor = Cursor::new([0u8; 4]);

    cursor.write_u16_be(0x1234).unwrap();
    cursor.write_u16_be(0x5678).unwrap();

    assert_eq!(cursor.get_ref(), &[0x12, 0x34, 0x56, 0x78]);

    // overflow
    let val = cursor.write_u16_be(0);
    assert!(val.is_err());
}

#[test]
fn write_u32_ne() {
    let mut cursor = Cursor::new([0u8; 8]);

    let parts = match cfg!(target_endian = "big") {
        true => (0x12345678, 0x9ABCDEF0),
        false => (0x78563412, 0xF0DEBC9A),
    };

    cursor.write_u32_ne(parts.0).unwrap();
    cursor.write_u32_ne(parts.1).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
    );

    // overflow
    let val = cursor.write_u32_ne(0);
    assert!(val.is_err());
}

#[test]
fn write_u32_le() {
    let mut cursor = Cursor::new([0u8; 8]);

    cursor.write_u32_le(0x78563412).unwrap();
    cursor.write_u32_le(0xF0DEBC9A).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
    );

    // overflow
    let val = cursor.write_u32_le(0);
    assert!(val.is_err());
}

#[test]
fn write_u32_be() {
    let mut cursor = Cursor::new([0u8; 8]);

    cursor.write_u32_be(0x12345678).unwrap();
    cursor.write_u32_be(0x9ABCDEF0).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]
    );

    // overflow
    let val = cursor.write_u32_be(0);
    assert!(val.is_err());
}

#[test]
fn write_u64_ne() {
    let mut cursor = Cursor::new([0u8; 16]);

    let parts = match cfg!(target_endian = "big") {
        true => (0x123456789ABCDEF0, 0x0123456789ABCDEF),
        false => (0xF0DEBC9A78563412, 0xEFCDAB8967452301),
    };

    cursor.write_u64_ne(parts.0).unwrap();
    cursor.write_u64_ne(parts.1).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF
        ]
    );

    // overflow
    let val = cursor.write_u64_ne(0);
    assert!(val.is_err());
}

#[test]
fn write_u64_le() {
    let mut cursor = Cursor::new([0u8; 16]);

    cursor.write_u64_le(0xF0DEBC9A78563412).unwrap();
    cursor.write_u64_le(0xEFCDAB8967452301).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF,
        ]
    );

    // overflow
    let val = cursor.write_u64_le(0);
    assert!(val.is_err());
}

#[test]
fn write_u64_be() {
    let mut cursor = Cursor::new([0u8; 16]);

    cursor.write_u64_be(0x123456789ABCDEF0).unwrap();
    cursor.write_u64_be(0x0123456789ABCDEF).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB,
            0xCD, 0xEF,
        ]
    );

    // overflow
    let val = cursor.write_u64_be(0);
    assert!(val.is_err());
}

#[test]
fn write_u128_ne() {
    let mut cursor = Cursor::new([0u8; 32]);

    let parts = match cfg!(target_endian = "big") {
        true => (
            0x01000000000000000000000000000000,
            0x01000000000000000000000000000000,
        ),
        false => (
            0x00000000000000000000000000000001,
            0x00000000000000000000000000000001,
        ),
    };

    cursor.write_u128_ne(parts.0).unwrap();
    cursor.write_u128_ne(parts.1).unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    );

    // overflow
    let val = cursor.write_u128_ne(0);
    assert!(val.is_err());
}

#[test]
fn write_u128_le() {
    let mut cursor = Cursor::new([0u8; 32]);

    cursor
        .write_u128_le(0x00000000000000000000000000000001)
        .unwrap();
    cursor
        .write_u128_le(0x00000000000000000000000000000001)
        .unwrap();

    assert_eq!(
        cursor.get_ref(),
        &[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    );

    // overflow
    let val = cursor.write_u128_le(0);
    assert!(val.is_err());
}

// TODO: Implement write_u128_be for all

#[test]
fn write_vec() {
    let mut cursor = Cursor::new([0u8; 8]);

    cursor.write_vec(vec![0, 1, 2, 3, 4, 5, 6, 7]).unwrap();

    assert_eq!(cursor.get_ref(), &[0, 1, 2, 3, 4, 5, 6, 7]);

    // overflow
    let val = cursor.write_vec(vec![0]);
    assert!(val.is_err());
}

#[test]
fn write_str() {
    let mut cursor = Cursor::new([0u8; 8]);

    cursor.write_str("ABCDEFGH".to_string()).unwrap();

    assert_eq!(cursor.get_ref(), b"ABCDEFGH");

    // overflow
    let val = cursor.write_str("A".to_string());
    assert!(val.is_err());
}
