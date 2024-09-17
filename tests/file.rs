use dh::{file, Readable, Writable};
use std::fs::remove_file;

#[test]
fn r000() {
    let path = "tests/samples/000";
    let mut reader = file::open_r(path).unwrap();
    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");
}

#[test]
fn w000() {
    let path = "tests/samples/w000";
    let mut writer = file::open_w(path).unwrap();
    let str = String::from("Hello, world!");
    writer.lock().unwrap(); // not necessary, but this prevents other processes from accessing the file
    writer.alloc(str.len() as u64).unwrap(); // not necessary, but it reserves space on the disk
    writer.write_utf8_at(&str, 0).unwrap();
    writer.close().unwrap();

    let mut reader = file::open_r(path).unwrap();
    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), str);
    reader.close().unwrap();

    remove_file(path).unwrap();
}
