use msgpack::{self, Value};
use std::io;

#[test]
fn unpacker() {
    let mut reader = io::Cursor::new(vec![
        0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
    ]);

    let mut unpacker = msgpack::Unpacker::new(&mut reader);

    assert_eq!(unpacker.unpack_nil().unwrap(), None as Option<usize>);
    assert_eq!(unpacker.unpack_bool().unwrap(), false);
    assert_eq!(unpacker.unpack_u8().unwrap(), 1);
    assert_eq!(unpacker.unpack_i8().unwrap(), -1);

    let len = unpacker.unpack_ary_header().unwrap();
    assert_eq!(len, 2);
    assert_eq!(unpacker.unpack_u8().unwrap(), 1);
    assert_eq!(unpacker.unpack_u8().unwrap(), 2);

    let len = unpacker.unpack_map_header().unwrap();
    assert_eq!(len, 1);
    assert_eq!(unpacker.unpack_u8().unwrap(), 1);
    assert_eq!(unpacker.unpack_string().unwrap(), "s".to_owned());
}

#[test]
fn unpacker_iter() {
    let mut reader = io::Cursor::new(vec![
        0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
    ]);
    let mut unpacker = msgpack::Unpacker::new(&mut reader);

    assert_eq!(unpacker.next().unwrap(), Value::Nil);
    assert_eq!(unpacker.next().unwrap(), Value::Boolean(false));
    assert_eq!(unpacker.next().unwrap(), Value::from(1 as u8));
    assert_eq!(unpacker.next().unwrap(), Value::from(-1 as i8));
    assert_eq!(
        unpacker.next().unwrap(),
        Value::Array(vec![Value::from(1 as u8), Value::from(2 as u8)])
    );
    assert_eq!(
        unpacker.next().unwrap(),
        Value::Map(vec![(Value::from(1 as u8), Value::String("s".to_owned()))])
    );
}
