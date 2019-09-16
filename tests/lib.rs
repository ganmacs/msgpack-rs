use msgpack;

use msgpack::{MessagePacker, MessageUnpacker};
use std::io;

#[test]
fn packer_unpacker() {
    let mut buf = vec![];
    let mut packer = msgpack::Packer::new(&mut buf);

    packer.pack_nil().unwrap();
    packer.pack_bool(false).unwrap();
    packer.pack_uint(1).unwrap();
    packer.pack_int(-1).unwrap();

    packer.pack_array_header(2).unwrap();
    packer.pack_uint(1).unwrap();
    packer.pack_uint(2).unwrap();

    packer.pack_map_header(1).unwrap();
    packer.pack_uint(1).unwrap();
    packer.pack_str("s").unwrap();

    let mut reader = io::Cursor::new(buf);
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
