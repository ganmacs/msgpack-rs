use std::io;

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
