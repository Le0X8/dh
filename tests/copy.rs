use dh::{ReadVal, ReadValAt};
use std::io::Cursor;

#[test]
fn copy_from_to() {
    let mut source = Cursor::new([1u8; 8]);
    let mut target = Cursor::new([0u8; 8]);

    source.copy(4, &mut target).unwrap();

    assert_eq!(target.get_ref(), &[1, 1, 1, 1, 0, 0, 0, 0]);
}

#[test]
fn copy_frompos_to() {
    let mut source = Cursor::new([0u8, 0, 0, 0, 1, 1, 1, 1]);
    let mut target = Cursor::new([0u8; 8]);

    source.copy_at(4, 2, &mut target).unwrap();

    assert_eq!(target.get_ref(), &[1, 1, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn copy_from_topos() {
    let mut source = Cursor::new([1u8; 8]);
    let mut target = Cursor::new([0u8; 8]);

    source.copy_to(4, 2, &mut target).unwrap();

    assert_eq!(target.get_ref(), &[0, 0, 1, 1, 1, 1, 0, 0]);
}

#[test]
fn copy_frompos_topos() {
    let mut source = Cursor::new([0u8, 0, 0, 0, 1, 1, 1, 1]);
    let mut target = Cursor::new([0u8; 8]);

    source.copy_to_at(4, 2, 2, &mut target).unwrap();

    assert_eq!(target.get_ref(), &[0, 0, 1, 1, 0, 0, 0, 0]);
}
