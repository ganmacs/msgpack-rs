use msgpack::{self, MessageUnpacker};
use std::io;

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

    let len = unpacker.unpack_array_header().unwrap();
    assert_eq!(len, 2);
    assert_eq!(unpacker.unpack_u8().unwrap(), 1);
    assert_eq!(unpacker.unpack_u8().unwrap(), 2);

    let len = unpacker.unpack_map_header().unwrap();
    assert_eq!(len, 1);
    assert_eq!(unpacker.unpack_u8().unwrap(), 1);
    assert_eq!(unpacker.unpack_string().unwrap(), "s".to_owned());
}
