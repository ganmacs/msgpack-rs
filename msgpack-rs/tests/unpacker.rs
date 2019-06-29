use msgpack::{self, BufferedRead, RefUnpacker, RefValue, Value};
use std::io::{self, Write};

#[test]
fn unpacker() {
    let mut reader = io::Cursor::new(vec![
        0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
    ]);

    let mut unpacker = msgpack::Unpacker::from_reader(&mut reader);

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
fn unpacker_new_write_and_read() {
    let val = vec![
        0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
    ];

    let mut unpacker = msgpack::Unpacker::new();
    unpacker.write(val.as_ref()).unwrap();
    assert_eq!(unpacker.unpack_value().unwrap(), Value::Nil);
    assert_eq!(unpacker.unpack_value().unwrap(), Value::Boolean(false));
    assert_eq!(unpacker.unpack_value().unwrap(), Value::from(1 as u8));
    assert_eq!(unpacker.unpack_value().unwrap(), Value::from(-1 as i8));
    assert_eq!(
        unpacker.unpack_value().unwrap(),
        Value::Array(vec![Value::from(1 as u8), Value::from(2 as u8)])
    );
    assert_eq!(
        unpacker.unpack_value().unwrap(),
        Value::Map(vec![(Value::from(1 as u8), Value::String("s".to_owned()))])
    );
}

#[test]
fn unpacker_iter() {
    let val = vec![0xc0, 0xc2, 0x01];
    let mut unpacker = msgpack::Unpacker::new();
    unpacker.write(val.as_ref()).unwrap();

    let mut items = unpacker.iter();
    assert_eq!(items.next().unwrap(), Value::Nil);
    assert_eq!(items.next().unwrap(), Value::Boolean(false));
    assert_eq!(items.next().unwrap(), Value::from(1 as u8));

    unpacker.write(val.as_ref()).unwrap();
    let mut items = unpacker.iter();
    assert_eq!(items.next().unwrap(), Value::Nil);
    assert_eq!(items.next().unwrap(), Value::Boolean(false));
    assert_eq!(items.next().unwrap(), Value::from(1 as u8));
}

// #[test]
// fn feed_slice() {
//     // let val = vec![
//     //     0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
//     // ];
//     let val = vec![0xc2];

//     let mut v = vec![];
//     let mut reader = io::Cursor::new(&mut v);
//     let mut unpacker = msgpack::Unpacker::new(&mut reader);
//     unpacker.feed_slice(val.as_ref()).unwrap();

//     // let mut v = unpacker.iter();
//     // println!("{:?}", v.next());
//     // assert_eq!(, Value::Nil);
//     // assert_eq!(v.next().unwrap(), Value::Boolean(false));
//     // assert_eq!(v.next().unwrap(), Value::from(1 as u8));
//     // assert_eq!(v.next().unwrap(), Value::from(-1 as i8));
//     // assert_eq!(
//     //     v.next().unwrap(),
//     //     RefValue::Array(vec![RefValue::from(1 as u8), RefValue::from(2 as u8)])
//     // );
//     // assert_eq!(
//     //     v.next().unwrap(),
//     //     RefValue::Map(vec![(RefValue::from(1 as u8), RefValue::String("s"))])
//     // );
// }

#[test]
fn feed_slice_with_pos() {
    let val = vec![
        0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73,
    ];
    let mut reader = io::Cursor::new(val.as_ref());

    let mut items = RefUnpacker::feed(&mut reader);
    assert_eq!(items.next().unwrap(), RefValue::Nil);
    assert_eq!(items.next().unwrap(), RefValue::Boolean(false));
    assert_eq!(items.next().unwrap(), RefValue::from(1 as u8));
    assert_eq!(items.next().unwrap(), RefValue::from(-1 as i8));
    assert_eq!(
        items.next().unwrap(),
        RefValue::Array(vec![RefValue::from(1 as u8), RefValue::from(2 as u8)])
    );
    assert_eq!(
        items.next().unwrap(),
        RefValue::Map(vec![(RefValue::from(1 as u8), RefValue::String("s"))])
    );
}
