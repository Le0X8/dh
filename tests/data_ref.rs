use dh::{data, Readable, Rw, Writable};

#[test]
fn r000() {
    let data = "Hello, world!".as_bytes().to_vec();
    let data_ref = &data;
    let mut reader = data::read_ref(&data);

    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");

    assert_eq!(data::close_ref(reader).unwrap(), data_ref);
}

#[test]
fn r001() {
    let data = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let data2 = {
        let mut reader = data::read_ref(&data);

        assert_eq!(reader.read_u8_at(0).unwrap(), 0);
        assert_eq!(reader.read_u16le_at(6).unwrap(), 0x0706);
        assert_eq!(reader.read_u64be().unwrap(), 0x0001020304050607);

        data::close_ref(reader).unwrap()
    };
    assert_eq!(data2, &data);
}

#[test]
fn w000() {
    let mut data = "Hello, world!".as_bytes().to_vec();
    let mut writer = data::write_ref(&mut data);

    writer.write_utf8_at(7, &"rust ".to_owned()).unwrap();

    data::close_ref(writer).unwrap(); // close_mut would return the mutable reference
    assert_eq!(data, "Hello, rust !".as_bytes());
}

#[test]
fn w001() {
    let mut data = "Hello, world!".as_bytes().to_vec();
    let mut writer = data::write_ref(&mut data);

    let size = writer.size().unwrap();
    writer.alloc(size + 5).unwrap(); // not necessary but it reserves RAM and prevents reallocation
    writer.write_utf8_at(7, &"rust world!".to_owned()).unwrap();

    writer.close().unwrap(); // recommended if the data is not used anymore
    assert_eq!(data, "Hello, rust world!".as_bytes());
}

#[test]
fn rw000() {
    let mut data = vec![0u8; 2];
    let mut rw = data::rw_ref(&mut data);

    rw.write_u16be(0x1234).unwrap();
    rw.rw_rewind().unwrap();
    assert_eq!(rw.read_u16be().unwrap(), 0x1234);

    rw.rw_close().unwrap();
    assert_eq!(data, vec![0x12, 0x34]);
}
