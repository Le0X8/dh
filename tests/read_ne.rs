use dh::{ReadValNe, Result};
use std::io::{Cursor, Read};

#[test]
fn read_borrowing() {
    let data = [0u8, 1, 2, 3];
    let mut cursor = Cursor::new(data);

    // this just needs to compile
    let mut borrowed: &mut dyn Read = &mut cursor;
    assert_eq!(borrowed.read_u8().unwrap(), 0);
    assert_eq!(borrowed.read_u8().unwrap(), 1);
    assert_eq!(cursor.read_u8().unwrap(), 2);
    assert_eq!(cursor.read_u8().unwrap(), 3);
}

#[test]
fn read_primitive() {
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut cursor = Cursor::new(data);

    let val: u8 = cursor.read_ne().unwrap();
    assert_eq!(val, 0);

    let val: [u8; 3] = cursor.read_ne().unwrap();
    assert_eq!(val, [1, 2, 3]);

    let val: u16 = cursor.read_ne().unwrap();
    assert_eq!(
        val,
        match cfg!(target_endian = "big") {
            true => 0x04_05,
            false => 0x05_04,
        }
    );

    let val: u16 = cursor.read_ne().unwrap();
    assert_eq!(
        val,
        match cfg!(target_endian = "big") {
            true => 0x06_07,
            false => 0x07_06,
        }
    );

    // overflow
    let val: Result<u8> = cursor.read_ne();
    assert!(val.is_err());
}

#[test]
fn read_u8() {
    let data = [0u8, 1, 2, 3];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u8().unwrap(), 0);
    assert_eq!(cursor.read_u8().unwrap(), 1);
    assert_eq!(cursor.read_u8().unwrap(), 2);
    assert_eq!(cursor.read_u8().unwrap(), 3);

    // overflow
    let val = cursor.read_u8();
    assert!(val.is_err());
}

#[test]
fn read_u16_ne() {
    let data = [0x12u8, 0x34, 0x56, 0x78];
    let mut cursor = Cursor::new(data);

    let parts = match cfg!(target_endian = "big") {
        true => (0x1234, 0x5678),
        false => (0x3412, 0x7856),
    };

    assert_eq!(cursor.read_u16().unwrap(), parts.0);
    assert_eq!(cursor.read_u16().unwrap(), parts.1);

    // overflow
    let val = cursor.read_u16();
    assert!(val.is_err());
}

#[test]
fn read_u32_ne() {
    let data = [0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let mut cursor = Cursor::new(data);

    let parts = match cfg!(target_endian = "big") {
        true => (0x12345678, 0x9ABCDEF0),
        false => (0x78563412, 0xF0DEBC9A),
    };

    assert_eq!(cursor.read_u32().unwrap(), parts.0);
    assert_eq!(cursor.read_u32().unwrap(), parts.1);

    // overflow
    let val = cursor.read_u32();
    assert!(val.is_err());
}

#[test]
fn read_u64_ne() {
    let data = [
        0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF,
    ];
    let mut cursor = Cursor::new(data);

    let parts = match cfg!(target_endian = "big") {
        true => (0x123456789ABCDEF0, 0x0123456789ABCDEF),
        false => (0xF0DEBC9A78563412, 0xEFCDAB8967452301),
    };

    assert_eq!(cursor.read_u64().unwrap(), parts.0);
    assert_eq!(cursor.read_u64().unwrap(), parts.1);

    // overflow
    let val = cursor.read_u64();
    assert!(val.is_err());
}

#[test]
fn read_u128_ne() {
    let data = [
        1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let mut cursor = Cursor::new(data);

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

    assert_eq!(cursor.read_u128().unwrap(), parts.0);
    assert_eq!(cursor.read_u128().unwrap(), parts.1);

    // overflow
    let val = cursor.read_u128();
    assert!(val.is_err());
}

#[test]
fn read_vec() {
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut cursor = Cursor::new(data);

    let val: Vec<u8> = cursor.read_vec(4).unwrap();
    assert_eq!(val, vec![0, 1, 2, 3]);

    // overflow
    let val: Result<Vec<u8>> = cursor.read_vec(10);
    assert!(val.is_err());
}

#[test]
fn read_str() {
    let data = [0x41u8, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48];
    let mut cursor = Cursor::new(data);

    let val = cursor.read_str(4).unwrap();
    assert_eq!(val, String::from("ABCD"));

    // overflow
    let val: Result<String> = cursor.read_str(10);
    assert!(val.is_err());
}
