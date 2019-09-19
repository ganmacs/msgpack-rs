use msgpack_serde;
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
    assert_eq!(vec![0xc3], msgpack_serde::pack(&true).unwrap());
    assert_eq!(vec![0xc2], msgpack_serde::pack(&false).unwrap());
}

#[test]
fn ser_option() {
    let v: Option<u8> = None;
    let v2: Option<u8> = Some(1);
    assert_eq!(vec![0xc0], msgpack_serde::pack(&v).unwrap());
    assert_eq!(vec![0x01], msgpack_serde::pack(&v2).unwrap());
}

#[test]
fn ser_u8() {
    let vals: Vec<u8> = vec![1, 127, 128, 255];
    assert_eq!(vec![0x01], msgpack_serde::pack(&vals[0]).unwrap());
    assert_eq!(vec![0x7f], msgpack_serde::pack(&vals[1]).unwrap());
    assert_eq!(vec![0xcc, 0x80], msgpack_serde::pack(&vals[2]).unwrap());
    assert_eq!(vec![0xcc, 0xff], msgpack_serde::pack(&vals[3]).unwrap());
}

#[test]
fn ser_str() {
    assert_eq!(
        vec![0xa3, 0x61, 0x61, 0x61],
        msgpack_serde::pack("aaa").unwrap()
    );
}

#[test]
fn ser_string() {
    assert_eq!(
        vec![0xa3, 0x61, 0x61, 0x61],
        msgpack_serde::pack(&"aaa".to_string()).unwrap()
    );
}

#[test]
fn ser_array() {
    let v = vec![1, 2, 3];
    assert_eq!(
        vec![0x93, 0x01, 0x02, 0x03],
        msgpack_serde::pack(&v).unwrap()
    );
}

#[test]
fn ser_unit_struct() {
    assert_eq!(vec![0x90], msgpack_serde::pack(&TestA {}).unwrap());
}

#[test]
fn ser_unit_variant() {
    assert_eq!(
        vec![0x81, 0xa1, 0x41, 0xc0],
        msgpack_serde::pack(&TestEnum::A).unwrap()
    );
}

#[test]
fn ser_newtype_struct() {
    assert_eq!(vec![0x0a], msgpack_serde::pack(&TestStructB(10)).unwrap());
}

#[test]
fn ser_newtype_variant() {
    assert_eq!(
        vec![0x81, 0xa1, 0x41, 0x0a],
        msgpack_serde::pack(&TestEnum2::A(10)).unwrap()
    );
}

#[test]
fn ser_tuple() {
    assert_eq!(
        vec![0x93, 0x01, 0x02, 0x03],
        msgpack_serde::pack(&(1, 2, 3)).unwrap()
    );
}

#[test]
fn ser_tuple_struct() {
    assert_eq!(
        vec![0x92, 0x01, 0x02],
        msgpack_serde::pack(&TestStructC(1, 2)).unwrap()
    );
}

#[test]
fn ser_tuple_variant() {
    assert_eq!(
        vec![0x81, 0xa1, 0x41, 0x92, 0x01, 0x02],
        msgpack_serde::pack(&TestEnumC::A(1, 2)).unwrap()
    );
}

#[test]
fn ser_struct() {
    let d = TestStructD { a: 10 };
    assert_eq!(
        vec![0x81, 0xa1, 0x61, 0x0a],
        msgpack_serde::pack(&d).unwrap()
    );
}

#[test]
fn ser_struct_variant() {
    let d = TestEnumD::A { a: 10 };
    assert_eq!(
        vec![0x81, 0xa1, 0x41, 0x81, 0xa1, 0x61, 0x0a],
        msgpack_serde::pack(&d).unwrap()
    );
}
