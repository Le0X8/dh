use dh::{file, Readable, Writable};
use std::fs::remove_file;

#[test]
fn r000() {
    let path = "tests/samples/000";
    let mut reader = file::open_r(path).unwrap();
    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");

    assert_eq!(reader.read_u8().unwrap(), 0x48);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u16le().unwrap(), 0x6548);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u16be().unwrap(), 0x4865);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u32le().unwrap(), 0x6c6c6548);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u32be().unwrap(), 0x48656c6c);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u64le().unwrap(), 0x77202c6f6c6c6548);
    reader.rewind().unwrap();

    assert_eq!(reader.read_u64be().unwrap(), 0x48656c6c6f2c2077);
    reader.rewind().unwrap();

    assert_eq!(reader.read_uxle(3).unwrap(), 0x6c6548);
    reader.rewind().unwrap();

    assert_eq!(reader.read_uxle(7).unwrap(), 0x202c6f6c6c6548);
    reader.rewind().unwrap();

    assert_eq!(reader.read_uxbe(3).unwrap(), 0x48656c);
    reader.rewind().unwrap();

    assert_eq!(reader.read_uxbe(7).unwrap(), 0x48656c6c6f2c20);
    reader.rewind().unwrap();
}

#[test]
fn r001() {
    let path = "tests/samples/001";
    let mut reader = file::open_r(path).unwrap();

    assert_eq!(reader.read_vu7().unwrap(), 0b1101100_1100101_1001000);
    reader.rewind().unwrap();

    assert_eq!(reader.read_vu7r().unwrap(), 0b1001000_1100101_1101100);
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vu15le().unwrap(),
        0b010110011001111_110110001101100_110010111001000
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vu15be().unwrap(),
        0b110110011101100_100100011100101
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vu15ler().unwrap(),
        0b110010111001000_110110001101100_010110011001111
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vu15ber().unwrap(),
        0b100100011100101_110110011101100
    );
    reader.rewind().unwrap();

    assert_eq!(reader.read_i8().unwrap(), -56);
    reader.rewind().unwrap();

    assert_eq!(reader.read_i16le().unwrap(), -6712);
    reader.rewind().unwrap();

    assert_eq!(reader.read_i16be().unwrap(), -14107);
    reader.rewind().unwrap();

    assert_eq!(reader.read_vi7().unwrap(), -0b0101100_1100101_1001000);
    reader.rewind().unwrap();

    assert_eq!(reader.read_vi7r().unwrap(), -0b0001000_1100101_1101100);
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vi15le().unwrap(),
        0b010110011001111_110110001101100_110010111001000
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vi15be().unwrap(),
        -0b010110011101100_100100011100101
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vi15ler().unwrap(),
        -0b010010111001000_110110001101100_010110011001111
    );
    reader.rewind().unwrap();

    assert_eq!(
        reader.read_vi15ber().unwrap(),
        -0b000100011100101_110110011101100
    );
    reader.rewind().unwrap();
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
