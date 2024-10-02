use dh::{file, recommended::*};
use std::fs::remove_file;

#[test]
fn r000() {
    let path = "tests/samples/000";
    let mut reader = file::open_r(path).unwrap();
    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), "Hello, world!");

    assert_eq!(reader.read_u8_at(0).unwrap(), 0x48);

    assert_eq!(reader.read_u16le_at(0).unwrap(), 0x6548);

    assert_eq!(reader.read_u16be_at(0).unwrap(), 0x4865);

    assert_eq!(reader.read_u32le_at(0).unwrap(), 0x6c6c6548);

    assert_eq!(reader.read_u32be_at(0).unwrap(), 0x48656c6c);

    assert_eq!(reader.read_u64le_at(0).unwrap(), 0x77202c6f6c6c6548);

    assert_eq!(reader.read_u64be_at(0).unwrap(), 0x48656c6c6f2c2077);

    assert_eq!(reader.read_uxle_at(0, 3).unwrap(), 0x6c6548);

    assert_eq!(reader.read_uxle_at(0, 7).unwrap(), 0x202c6f6c6c6548);

    assert_eq!(reader.read_uxbe_at(0, 3).unwrap(), 0x48656c);

    assert_eq!(reader.read_uxbe_at(0, 7).unwrap(), 0x48656c6c6f2c20);
}

#[test]
fn r001() {
    let path = "tests/samples/001";
    let mut reader = file::open_r(path).unwrap();

    assert_eq!(reader.read_vu7_at(0).unwrap(), 0b1101100_1100101_1001000);

    assert_eq!(reader.read_vu7r_at(0).unwrap(), 0b1001000_1100101_1101100);

    assert_eq!(
        reader.read_vu15le_at(0).unwrap(),
        0b010110011001111_110110001101100_110010111001000
    );

    assert_eq!(
        reader.read_vu15be_at(0).unwrap(),
        0b110110011101100_100100011100101
    );

    assert_eq!(
        reader.read_vu15ler_at(0).unwrap(),
        0b110010111001000_110110001101100_010110011001111
    );

    assert_eq!(
        reader.read_vu15ber_at(0).unwrap(),
        0b100100011100101_110110011101100
    );

    assert_eq!(reader.read_i8_at(0).unwrap(), -56);

    assert_eq!(reader.read_i16le_at(0).unwrap(), -6712);

    assert_eq!(reader.read_i16be_at(0).unwrap(), -14107);

    assert_eq!(reader.read_vi7_at(0).unwrap(), -0b0101100_1100101_1001000);

    assert_eq!(reader.read_vi7r_at(0).unwrap(), -0b0001000_1100101_1101100);

    assert_eq!(
        reader.read_vi15le_at(0).unwrap(),
        0b010110011001111_110110001101100_110010111001000
    );

    assert_eq!(
        reader.read_vi15be_at(0).unwrap(),
        -0b010110011101100_100100011100101
    );

    assert_eq!(
        reader.read_vi15ler_at(0).unwrap(),
        -0b010010111001000_110110001101100_010110011001111
    );

    assert_eq!(
        reader.read_vi15ber_at(0).unwrap(),
        -0b000100011100101_110110011101100
    );
}

#[test]
fn w000() {
    let path = "tests/samples/w000";
    let mut writer = file::open_w(path).unwrap();
    let str = String::from("Hello, world!");
    writer.lock(false).unwrap(); // not necessary, but this prevents other processes from accessing the file
    writer.alloc(str.len() as u64).unwrap(); // not necessary, but it reserves space on the disk
    writer.write_utf8_at(0, &str).unwrap();
    writer.close().unwrap();

    let mut reader = file::open_r(path).unwrap();
    let size = reader.size().unwrap();
    assert_eq!(reader.read_utf8_at(0, size).unwrap(), str);
    reader.close().unwrap();

    remove_file(path).unwrap();
}

#[test]
fn w001() {
    let path = "tests/samples/w001";
    let mut writer = file::open_w(path).unwrap();
    writer.write_vu31le(0b1101100_1100101_1001000).unwrap();
    writer.write_vixler(3, 0b1101100_1100101_1001000).unwrap();
    writer.close().unwrap();

    let mut reader = file::open_r(path).unwrap();
    assert_eq!(reader.read_vuxle(4).unwrap(), 0b1101100_1100101_1001000);
    assert_eq!(reader.read_vixler(3).unwrap(), 0b1101100_1100101_1001000);
    reader.close().unwrap();

    remove_file(path).unwrap();
}

#[test]
fn rw000() {
    let path = "tests/samples/rw000";
    let mut rw = file::open_rw(path).unwrap();
    let str = String::from("Hello, world!");

    rw.write_utf8(&str).unwrap();
    rw.rewind().unwrap();
    assert_eq!(rw.read_utf8(str.len() as u64).unwrap(), str);
    rw.rw_close().unwrap();

    remove_file(path).unwrap();
}
