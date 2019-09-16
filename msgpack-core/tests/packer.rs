use msgpack::{self, MessagePacker};

#[test]
fn packer() {
    let mut writer = vec![];
    let mut packer = msgpack::Packer::new(&mut writer);

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

    assert_eq!(
        writer,
        &[0xc0, 0xc2, 0x01, 0xff, 0x92, 0x01, 0x02, 0x81, 0x01, 0xa1, 0x73]
    );
}
