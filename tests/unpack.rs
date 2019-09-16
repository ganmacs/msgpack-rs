use msgpack;
use msgpack::Value;
use std::io;

#[test]
fn read_bool() {
    let mut reader = io::Cursor::new(&[0xc3, 0xc2]);
    assert_eq!(msgpack::unpack_bool(&mut reader).unwrap(), true);
    assert_eq!(msgpack::unpack_bool(&mut reader).unwrap(), false);
}

#[test]
fn read_nil() {
    let mut reader = io::Cursor::new(&[0xc0]);
    assert_eq!(
        msgpack::unpack_nil(&mut reader).unwrap(),
        None as Option<u8>
    );
}

#[test]
fn unpack_u8() {
    let mut reader = io::Cursor::new(&[
        0xcc, 0x01, // 1
        0xcc, 0x7f, // 127
        0xcc, 0x80, // 128
        0xcc, 0xff, // 255
    ]);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 1);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 127);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 128);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 255);
}

#[test]
fn unpack_u16() {
    let mut reader = io::Cursor::new(&[
        0xcd, 0x01, 0x00, // 256
        0xcd, 0xff, 0xff, // 65535
    ]);
    assert_eq!(msgpack::unpack_u16(&mut reader).unwrap(), 256);
    assert_eq!(msgpack::unpack_u16(&mut reader).unwrap(), 65535);
}

#[test]
fn unpack_u32() {
    let mut reader = io::Cursor::new(&[
        0xce, 0x00, 0x01, 0x00, 0x00, // 65536
        0xce, 0xff, 0xff, 0xff, 0xff, // 4294967295
    ]);
    assert_eq!(msgpack::unpack_u32(&mut reader).unwrap(), 65536);
    assert_eq!(msgpack::unpack_u32(&mut reader).unwrap(), 4294967295);
}

#[test]
fn unpack_u64() {
    let mut reader = io::Cursor::new(&[
        0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // 4294967296
        0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 18446744073709551615
    ]);
    assert_eq!(msgpack::unpack_u64(&mut reader).unwrap(), 4294967296);
    assert_eq!(
        msgpack::unpack_u64(&mut reader).unwrap(),
        18446744073709551615
    );
}
#[test]
fn unpack_i8() {
    let mut reader = io::Cursor::new(&[
        0xff, // -1
        0xe0, // -32
        0xd0, 0xdf, // -33
        0xd0, 0x80, // -128
    ]);
    assert_eq!(msgpack::unpack_i8(&mut reader).unwrap(), -1);
    assert_eq!(msgpack::unpack_i8(&mut reader).unwrap(), -32);
    assert_eq!(msgpack::unpack_i8(&mut reader).unwrap(), -33);
    assert_eq!(msgpack::unpack_i8(&mut reader).unwrap(), -128);
}

#[test]
fn unpack_i16() {
    let mut reader = io::Cursor::new(&[
        0xd1, 0xff, 0x7f, // -129
        0xd1, 0x80, 0x00, // -32768
    ]);
    assert_eq!(msgpack::unpack_i16(&mut reader).unwrap(), -129);
    assert_eq!(msgpack::unpack_i16(&mut reader).unwrap(), -32768);
}

#[test]
fn unpack_i32() {
    let mut reader = io::Cursor::new(&[
        0xd2, 0xff, 0xff, 0x7f, 0xff, // -32769
        0xd2, 0x80, 0x00, 0x00, 0x00, // -2147483648
    ]);
    assert_eq!(msgpack::unpack_i32(&mut reader).unwrap(), -32769);
    assert_eq!(msgpack::unpack_i32(&mut reader).unwrap(), -2147483648);
}

#[test]
fn unpack_i64() {
    let mut reader = io::Cursor::new(&[
        0xd3, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, // -2147483649
        0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // -9223372036854775808
    ]);
    assert_eq!(msgpack::unpack_i64(&mut reader).unwrap(), -2147483649);
    assert_eq!(
        msgpack::unpack_i64(&mut reader).unwrap(),
        -9223372036854775808
    );
}

#[test]
fn unpack_bin() {
    let mut reader = io::Cursor::new(&[0xc4, 0x03, 0x61, 0x61, 0x61]);
    let len = msgpack::unpack_bin_header(&mut reader).unwrap();
    assert_eq!(len, 3);
    assert_eq!(
        msgpack::unpack_bin_data(&mut reader, len).unwrap(),
        &[0x61, 0x61, 0x61]
    );
}

#[test]
fn unpack_str() {
    let mut reader = io::Cursor::new(&[0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    assert_eq!(
        msgpack::unpack_str(&mut reader).unwrap(),
        "hello".to_string()
    );
    let mut reader = io::Cursor::new(&[0xd9, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    assert_eq!(
        msgpack::unpack_str(&mut reader).unwrap(),
        "hello".to_string()
    );
    let mut reader = io::Cursor::new(&[0xda, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    assert_eq!(
        msgpack::unpack_str(&mut reader).unwrap(),
        "hello".to_string()
    );
    let mut reader = io::Cursor::new(&[0xdb, 0x00, 0x00, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    assert_eq!(
        msgpack::unpack_str(&mut reader).unwrap(),
        "hello".to_string()
    );
}

#[test]
fn unpack_fixary() {
    let mut reader = io::Cursor::new(&[0x92, 0x01, 0x02]);
    assert_eq!(msgpack::unpack_array_header(&mut reader).unwrap(), 2);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 1);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 2);
}

#[test]
fn unpack_fixmap() {
    let mut reader = io::Cursor::new(&[0x81, 0x01, 0x02]);
    assert_eq!(msgpack::unpack_map_header(&mut reader).unwrap(), 1);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 1);
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 2);
}

#[test]
fn unpack_ext() {
    let mut reader = io::Cursor::new(&[0xd4, 0x01, 0x02]);
    assert_eq!(msgpack::unpack_ext_header(&mut reader).unwrap(), (1, 1));
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 2);
}

#[test]
fn unpack_fixext1() {
    let mut reader = io::Cursor::new(&[0xd4, 0x01, 0x02]);
    assert_eq!(msgpack::unpack_fixext1(&mut reader).unwrap(), (1, 2));
}

#[test]
fn unpack_fixext2() {
    let mut reader = io::Cursor::new(&[0xd5, 0x02, 0x02, 0x02]);
    assert_eq!(
        msgpack::unpack_fixext2(&mut reader).unwrap(),
        (2, [0x02, 0x02])
    );
}

#[test]
fn unpack_fixext4() {
    let mut reader = io::Cursor::new(&[0xd6, 0x04, 0x02, 0x02, 0x02, 0x02, 0x01]);
    assert_eq!(
        msgpack::unpack_fixext4(&mut reader).unwrap(),
        (4, [0x02, 0x02, 0x02, 0x02])
    );
    assert_eq!(msgpack::unpack_u8(&mut reader).unwrap(), 0x01);
}

#[test]
fn unpack_value() {
    let mut reader = io::Cursor::new([0xc3, 0xc2, 0xcc, 0x01, 0xc0, 0x92, 0x01, 0x02]);
    assert_eq!(
        msgpack::unpack_value(&mut reader).unwrap(),
        Value::Boolean(true)
    );

    assert_eq!(
        msgpack::unpack_value(&mut reader).unwrap(),
        Value::Boolean(false)
    );

    assert_eq!(
        msgpack::unpack_value(&mut reader).unwrap(),
        Value::from(1 as u8)
    );

    assert_eq!(msgpack::unpack_value(&mut reader).unwrap(), Value::Nil);

    assert_eq!(
        msgpack::unpack_value(&mut reader).unwrap(),
        Value::Array(vec![Value::from(1 as u8), Value::from(2 as u8)])
    );
}

#[test]
fn unpack_invalid_str_value() {
    let v: Vec<u8> = vec![
        219, 0, 0, 0, 52, 146, 215, 0, 93, 24, 122, 26, 4, 114, 82, 40, 129, 167, 109, 101, 115,
        115, 97, 103, 101, 165, 100, 117, 109, 109, 121, 146, 215, 0, 93, 24, 122, 27, 5, 181, 62,
        32, 129, 167, 109, 101, 115, 115, 97, 103, 101, 165, 100, 117, 109, 109, 121,
    ];

    let mut reader = io::Cursor::new(&v);

    let mut writer = vec![];
    let val = msgpack::unpack_value(&mut reader).unwrap();
    if let Value::String(ref s) = &val {
        assert!(s.is_err());
        msgpack::pack_value(&mut writer, val).unwrap();

        // right skip 1 bytes which is header (fixstring)
        // left: skip 4 bytes (219, 0, 0, 0, 52) which is header (str32)
        assert_eq!(writer[1..(writer.len())], v[4..(v.len())]);
    } else {
        panic!("must be string");
    };
}
