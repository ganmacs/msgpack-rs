use msgpack_serde;
use serde::Deserialize;

#[test]
fn de_bool() {
    assert_eq!(true, msgpack_serde::unpack(&[0xc3]).unwrap());
    assert_eq!(false, msgpack_serde::unpack(&[0xc2]).unwrap());
}

#[test]
fn de_option() {
    let expect1: Option<u8> = None;
    let expect2: Option<u8> = Some(1);

    assert_eq!(expect1, msgpack_serde::unpack(&[0xc0]).unwrap());
    assert_eq!(expect2, msgpack_serde::unpack(&[0x01]).unwrap());
}

#[test]
fn de_u8() {
    assert_eq!(1 as u8, msgpack_serde::unpack(&[0x01]).unwrap());
    assert_eq!(127 as u8, msgpack_serde::unpack(&[0x7f]).unwrap());
    assert_eq!(128 as u8, msgpack_serde::unpack(&[0xcc, 0x80]).unwrap());
    assert_eq!(255 as u8, msgpack_serde::unpack(&[0xcc, 0xff]).unwrap());
}

#[test]
fn de_u32() {
    assert_eq!(
        65536 as u32,
        msgpack_serde::unpack(&[0xce, 0x00, 0x01, 0x00, 0x00]).unwrap()
    );
    assert_eq!(
        4294967295 as u32,
        msgpack_serde::unpack(&[0xce, 0xff, 0xff, 0xff, 0xff]).unwrap()
    );
}

#[test]
fn de_seq() {
    let v2: Vec<u8> = msgpack_serde::unpack(&[0x92, 0x01, 0x02]).unwrap();
    assert_eq!(vec![1, 2], v2);
}

#[test]
fn de_tuple() {
    let buf = [0x92, 0x01, 0x02];
    let ret: (u8, u8) = msgpack_serde::unpack(&buf).unwrap();
    assert_eq!((1, 2), ret);
}

#[test]
fn de_string() {
    let buf = [0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let ret: String = msgpack_serde::unpack(&buf).unwrap();
    assert_eq!("hello".to_string(), ret);
}

#[test]
fn de_str() {
    use std::borrow::Cow;
    let buf = [0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let ret: Cow<str> = msgpack_serde::unpack(&buf).unwrap();
    assert_eq!(Cow::from("hello"), ret);
}

#[derive(Deserialize, Debug, PartialEq)]
struct Point {
    h: u8,
    o: String,
}

#[test]
fn de_struct() {
    let buf = [0x82, 0xa1, 0x68, 0x01, 0xa1, 0x6f, 0xa1, 0x6f];
    let ret: Point = msgpack_serde::unpack(&buf).unwrap();
    assert_eq!(
        Point {
            o: "o".to_string(),
            h: 1,
        },
        ret
    );
}
