use dh::{data, Readable};

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
