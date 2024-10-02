use dh::{data, recommended::*};

#[test]
fn r000() {
    let data = "Hello, world!".as_bytes().to_vec();
    let cloned = data.clone();
    let mut reader = data::read(data);

    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");

    assert_eq!(data::close(reader).unwrap(), cloned);
}

#[test]
fn r001() {
    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut reader = data::read(data);

    assert_eq!(reader.read_u8_at(0).unwrap(), 0);
    assert_eq!(reader.read_u16le_at(6).unwrap(), 0x0706);
    assert_eq!(reader.read_u64be().unwrap(), 0x0001020304050607);
}

#[test]
fn w000() {
    let data = "Hello, world!".as_bytes().to_vec();
    let mut writer = data::write(data);

    writer.write_utf8_at(7, &"rust ".to_owned()).unwrap();

    let data = data::close(writer).unwrap();
    assert_eq!(data, "Hello, rust !".as_bytes());
}

#[test]
fn w001() {
    let data = "Hello, world!".as_bytes().to_vec();
    let mut writer = data::write(data);

    let size = writer.size().unwrap();
    writer.alloc(size + 5).unwrap(); // not necessary but it reserves RAM and prevents reallocation
    writer.write_utf8_at(7, &"rust world!".to_owned()).unwrap();

    let data = data::close(writer).unwrap();
    assert_eq!(data, "Hello, rust world!".as_bytes());
}

#[test]
fn w002() {
    let mut writer = data::write_empty();

    writer.alloc(13).unwrap();
    writer
        .write_utf8_at(0, &"Hello, world!".to_owned())
        .unwrap();

    let data = data::close(writer).unwrap();
    assert_eq!(data, "Hello, world!".as_bytes());
}

#[test]
fn w003() {
    let mut writer = data::write_new(13); // better than w002

    writer
        .write_utf8_at(0, &"Hello, world!".to_owned())
        .unwrap();

    let data = data::close(writer).unwrap();
    assert_eq!(data, "Hello, world!".as_bytes());
}

#[test]
fn w004() {
    let buf = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = data::read_ref(&buf);

    let mut writer = data::write_new(8);
    reader.copy(8, &mut writer, 8000).unwrap(); // even if the limit is 8000, it will copy only 8 bytes

    let new_buf = data::close(writer).unwrap();
    assert_eq!(new_buf, buf);
}

#[test]
fn w005() {
    let buf = vec![1, 2, 3, 4];
    let mut reader = data::read(buf);

    let mut writer = data::write_new(8);
    reader.copy_to(4, 4, &mut writer, 4).unwrap();

    let new_buf = data::close(writer).unwrap();
    assert_eq!(new_buf, vec![0, 0, 0, 0, 1, 2, 3, 4]);
}

#[test]
fn w006() {
    let buf = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = data::read(buf);

    let mut writer = data::write_new(8);
    reader.copy_to_at(2, 2, 4, &mut writer, 4).unwrap();

    let new_buf = data::close(writer).unwrap();
    assert_eq!(new_buf, vec![0, 0, 3, 4, 5, 6, 0, 0]);
}

#[test]
fn rw000() {
    let mut rw = data::rw_new(2);

    rw.write_u16be(0x1234).unwrap();
    rw.rewind().unwrap();
    assert_eq!(rw.read_u16be().unwrap(), 0x1234);

    let data = data::close(rw).unwrap();
    assert_eq!(data, vec![0x12, 0x34]);
}
