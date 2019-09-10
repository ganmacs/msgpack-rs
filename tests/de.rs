use msgpack::{self, de};
use serde::Deserialize;

#[test]
fn de_bool() {
    let buf = [0xc3, 0xc2];
    let mut de = de::Deserializer::new(&buf[..]);

    assert_eq!(true, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(false, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn de_option() {
    let buf = [0xc0, 0x01];
    let mut de = de::Deserializer::new(&buf[..]);

    let expect1: Option<u8> = None;
    let expect2: Option<u8> = Some(1);

    assert_eq!(expect1, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(expect2, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn de_u8() {
    let buf = [
        0x01, // 1
        0x7f, // 127
        0xcc, 0x80, // 128
        0xcc, 0xff, // 255
    ];
    let mut de = de::Deserializer::new(&buf[..]);

    assert_eq!(1 as u8, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(127 as u8, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(128 as u8, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(255 as u8, Deserialize::deserialize(&mut de).unwrap());
}

#[test]
fn de_u32() {
    let buf = [
        0xce, 0x00, 0x01, 0x00, 0x00, // 65536
        0xce, 0xff, 0xff, 0xff, 0xff, // 4294967295
    ];
    let mut de = de::Deserializer::new(&buf[..]);

    assert_eq!(65536 as u32, Deserialize::deserialize(&mut de).unwrap());
    assert_eq!(
        4294967295 as u32,
        Deserialize::deserialize(&mut de).unwrap()
    );
}

#[test]
fn unpack_seq() {
    let buf = [0x92, 0x01, 0x02];
    let mut de = de::Deserializer::new(&buf[..]);
    let v2: Vec<u8> = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!(vec![1, 2], v2);
}

#[test]
fn unpack_tuple() {
    let buf = [0x92, 0x01, 0x02];
    let mut de = de::Deserializer::new(&buf[..]);
    let ret: (u8, u8) = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!((1, 2), ret);
}

#[test]
fn unpack_string() {
    let buf = [0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let mut de = de::Deserializer::new(&buf[..]);
    let ret: String = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!("hello".to_string(), ret);
}

#[test]
fn unpack_str() {
    use std::borrow::Cow;

    let buf = [0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let mut de = de::Deserializer::new(&buf[..]);
    let ret: Cow<str> = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!(Cow::from("hello"), ret);
}

#[derive(Deserialize, Debug, PartialEq)]
struct Point {
    h: u8,
    o: String,
}

#[test]
fn unpack_struct() {
    let buf = [0x82, 0xa1, 0x68, 0x01, 0xa1, 0x6f, 0xa1, 0x6f];
    let mut de = de::Deserializer::new(&buf[..]);
    let ret: Point = Deserialize::deserialize(&mut de).unwrap();
    assert_eq!(
        Point {
            o: "o".to_string(),
            h: 1,
        },
        ret
    );
}
