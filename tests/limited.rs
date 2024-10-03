use dh::recommended::*;

#[test]
fn r000() {
    let data = "Hello, world!".as_bytes().to_vec();

    let mut reader = dh::data::read(data);

    let mut limited = reader.limit(0, 5).unwrap();

    assert_eq!(limited.size().unwrap(), 5);

    limited.to(6).unwrap_err();

    limited.end().unwrap();
    assert_eq!(limited.pos().unwrap(), 5);

    reader.jump(2).unwrap();
    assert_eq!(reader.read_utf8(5).unwrap(), "world");

    let mut limited = reader.limit(7, 5).unwrap();

    limited.rewind().unwrap();
    assert_eq!(limited.pos().unwrap(), 0);
    assert_eq!(limited.jump(1).unwrap(), 1);
    assert_eq!(limited.jump(-1).unwrap(), 0);
    assert_eq!(reader.pos().unwrap(), 7);
}

#[test]
fn r001() {
    let data = "Hello, world!".as_bytes().to_vec();

    let mut reader = dh::data::read(data);

    let mut limited = reader.limit(0, 5).unwrap();

    let size = limited.size().unwrap();
    assert_eq!(limited.read_utf8(size).unwrap(), "Hello");

    let mut limited = reader.limit(7, 5).unwrap();
    limited.end().unwrap();

    limited.read_u8().unwrap_err();

    assert_eq!(reader.pos().unwrap(), 12);
}
