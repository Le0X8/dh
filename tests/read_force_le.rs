use dh::ForceReadValLe;
use std::io::{Cursor, Read};

#[test]
fn read_borrowing() {
    let data = [0u8, 1, 2, 3];
    let mut cursor = Cursor::new(data);

    // this just needs to compile
    let mut borrowed: &mut dyn Read = &mut cursor;
    assert_eq!(borrowed.read_u8(), 0);
    assert_eq!(borrowed.read_u8(), 1);
    assert_eq!(cursor.read_u8(), 2);
    assert_eq!(cursor.read_u8(), 3);
}

#[test]
fn read_primitive() {
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut cursor = Cursor::new(data);

    let val: u8 = cursor.read_le();
    assert_eq!(val, 0);

    let val: [u8; 3] = cursor.read_le();
    assert_eq!(val, [1, 2, 3]);

    let val: u16 = cursor.read_le();
    assert_eq!(val, 0x05_04);

    let val: u16 = cursor.read_le();
    assert_eq!(val, 0x07_06);
}

#[test]
fn read_u8() {
    let data = [0u8, 1, 2, 3];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u8(), 0);
    assert_eq!(cursor.read_u8(), 1);
    assert_eq!(cursor.read_u8(), 2);
    assert_eq!(cursor.read_u8(), 3);
}

#[test]
#[should_panic]
fn read_u8_panic() {
    let data = [0u8, 1, 2, 3];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u8(), 0);
    assert_eq!(cursor.read_u8(), 1);
    assert_eq!(cursor.read_u8(), 2);
    assert_eq!(cursor.read_u8(), 3);

    // overflow
    cursor.read_u8();
}

#[test]
fn read_u16_le() {
    let data = [0x12u8, 0x34, 0x56, 0x78];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u16(), 0x3412);
    assert_eq!(cursor.read_u16(), 0x7856);
}

#[test]
fn read_u32_le() {
    let data = [0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u32(), 0x78563412);
    assert_eq!(cursor.read_u32(), 0xF0DEBC9A);
}

#[test]
fn read_u64_le() {
    let data = [
        0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD,
        0xEF,
    ];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u64(), 0xF0DEBC9A78563412);
    assert_eq!(cursor.read_u64(), 0xEFCDAB8967452301);
}

#[test]
fn read_u128_le() {
    let data = [
        1u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let mut cursor = Cursor::new(data);

    assert_eq!(cursor.read_u128(), 0x00000000000000000000000000000001);
    assert_eq!(cursor.read_u128(), 0x00000000000000000000000000000001);
}

#[test]
fn read_vec() {
    let data = [0u8, 1, 2, 3, 4, 5, 6, 7];
    let mut cursor = Cursor::new(data);

    let val: Vec<u8> = cursor.read_vec(4);
    assert_eq!(val, vec![0, 1, 2, 3]);
}

#[test]
fn read_str() {
    let data = [0x41u8, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48];
    let mut cursor = Cursor::new(data);

    let val = cursor.read_str(4);
    assert_eq!(val, String::from("ABCD"));
}
