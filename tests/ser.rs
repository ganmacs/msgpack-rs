use msgpack;
use serde::Serialize;

#[derive(Serialize)]
struct TestA;

#[derive(Serialize)]
struct TestStructB(u8);

#[derive(Serialize)]
struct TestStructC(u8, u8);

#[derive(Serialize)]
struct TestStructD {
    a: u8,
}

#[derive(Serialize)]
enum TestEnum {
    A,
}

#[derive(Serialize)]
enum TestEnum2 {
    A(u8),
}

#[derive(Serialize)]
enum TestEnumC {
    A(u8, u8),
}

#[derive(Serialize)]
enum TestEnumD {
    A { a: u8 },
}

#[test]
fn ser_bool() {
    let mut writer = vec![];

    true.serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    false
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();

    assert_eq!([0xc3, 0xc2], writer[..]);
}

#[test]
fn ser_option() {
    let mut writer = vec![];
    let v: Option<u8> = None;
    let v2: Option<u8> = Some(1);
    v.serialize(&mut msgpack::Packer::new(&mut writer)).unwrap();
    v2.serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0xc0, 0x01], writer[..]);
}

#[test]
fn ser_u8() {
    let mut writer = vec![];
    let vals: Vec<u8> = vec![1, 127, 128, 255];

    for v in vals {
        v.serialize(&mut msgpack::Packer::new(&mut writer)).unwrap();
    }

    assert_eq!(
        [
            0x01, // 1
            0x7f, // 127
            0xcc, 0x80, // 128
            0xcc, 0xff, // 255
        ],
        writer[..]
    );
}

#[test]
fn ser_str() {
    let mut writer = vec![];
    let msg = "aaa";
    msg.serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0xa3, 0x61, 0x61, 0x61], writer[..]);
}

#[test]
fn ser_string() {
    let mut writer = vec![];
    let msg = "aaa".to_string();
    msg.serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0xa3, 0x61, 0x61, 0x61], writer[..]);
}

#[test]
fn ser_array() {
    let mut writer = vec![];
    let v = vec![1, 2, 3];
    v.serialize(&mut msgpack::Packer::new(&mut writer)).unwrap();
    assert_eq!([0x93, 0x01, 0x02, 0x03], writer[..]);
}

#[test]
fn ser_unit_struct() {
    let mut writer = vec![];
    (TestA {})
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x90], writer[..]);
}

#[test]
fn ser_unit_variant() {
    let mut writer = vec![];
    TestEnum::A
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x81, 0xa1, 0x41, 0xc0], writer[..]);
}

#[test]
fn ser_newtype_struct() {
    let mut writer = vec![];
    TestStructB(10)
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x0a], writer[..]);
}

#[test]
fn ser_newtype_variant() {
    let mut writer = vec![];
    TestEnum2::A(10)
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x81, 0xa1, 0x41, 0x0a], writer[..]);
}

#[test]
fn ser_tuple() {
    let mut writer = vec![];
    (1, 2, 3)
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x93, 0x01, 0x02, 0x03], writer[..]);
}

#[test]
fn ser_tuple_struct() {
    let mut writer = vec![];
    TestStructC(1, 2)
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x92, 0x01, 0x02], writer[..]);
}

#[test]
fn ser_tuple_variant() {
    let mut writer = vec![];
    TestEnumC::A(1, 2)
        .serialize(&mut msgpack::Packer::new(&mut writer))
        .unwrap();
    assert_eq!([0x81, 0xa1, 0x41, 0x92, 0x01, 0x02], writer[..]);
}

#[test]
fn ser_struct() {
    let mut writer = vec![];
    let d = TestStructD { a: 10 };
    d.serialize(&mut msgpack::Packer::new(&mut writer)).unwrap();
    assert_eq!([0x81, 0xa1, 0x61, 0x0a], writer[..]);
}

#[test]
fn ser_struct_variant() {
    let mut writer = vec![];
    let d = TestEnumD::A { a: 10 };
    d.serialize(&mut msgpack::Packer::new(&mut writer)).unwrap();
    assert_eq!([0x81, 0xa1, 0x41, 0x81, 0xa1, 0x61, 0x0a], writer[..]);
}
