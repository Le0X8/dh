use dh::{data, Readable};

#[test]
fn r000() {
    let data = "Hello, world!".as_bytes().to_vec();
    let mut reader = data::from_r(data);

    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");
}

#[test]
fn r001() {
    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut reader = data::from_r(data);

    assert_eq!(reader.read_u8_at(0).unwrap(), 0);
    assert_eq!(reader.read_u16le_at(6).unwrap(), 0x0706);
    assert_eq!(reader.read_u64be().unwrap(), 0x0001020304050607);
}
