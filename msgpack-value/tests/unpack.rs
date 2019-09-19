use std::io;

use msgpack_value::Value;

#[test]
fn unpack_value() {
    let mut reader = io::Cursor::new([0xc3, 0xc2, 0xcc, 0x01, 0xc0, 0x92, 0x01, 0x02]);
    assert_eq!(
        msgpack_value::unpack_value(&mut reader).unwrap(),
        Value::Boolean(true)
    );

    assert_eq!(
        msgpack_value::unpack_value(&mut reader).unwrap(),
        Value::Boolean(false)
    );

    assert_eq!(
        msgpack_value::unpack_value(&mut reader).unwrap(),
        Value::from(1 as u8)
    );

    assert_eq!(
        msgpack_value::unpack_value(&mut reader).unwrap(),
        Value::Nil
    );

    assert_eq!(
        msgpack_value::unpack_value(&mut reader).unwrap(),
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
    let val = msgpack_value::unpack_value(&mut reader).unwrap();
    if let Value::String(ref s) = &val {
        assert!(s.is_err());
        msgpack_value::pack_value(&mut writer, val).unwrap();

        // right skip 1 bytes which is header (fixstring)
        // left: skip 4 bytes (219, 0, 0, 0, 52) which is header (str32)
        assert_eq!(writer[1..(writer.len())], v[4..(v.len())]);
    } else {
        panic!("must be string");
    };
}
