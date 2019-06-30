use msgpack;

#[test]
fn pack_bool() {
    let mut writer = vec![];
    msgpack::pack_bool(&mut writer, true).unwrap();
    msgpack::pack_bool(&mut writer, false).unwrap();
    assert_eq!(writer, &[0xc3, 0xc2]);
}

#[test]
fn pack_nil() {
    let mut writer = vec![];
    msgpack::pack_nil(&mut writer).unwrap();
    assert_eq!(writer, &[0xc0]);
}

#[test]
fn pack_u8() {
    let mut writer = vec![];
    let vals = vec![1, 127, 128, 255];

    for v in vals {
        msgpack::pack_u8(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xcc, 0x01, // 1
            0xcc, 0x7f, // 127
            0xcc, 0x80, // 128
            0xcc, 0xff, // 255
        ]
    );
}

#[test]
fn pack_u16() {
    let mut writer = vec![];
    let vals = vec![255, 256, 65535];

    for v in vals {
        msgpack::pack_u16(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xcd, 0x00, 0xff, // 255
            0xcd, 0x01, 0x00, // 256
            0xcd, 0xff, 0xff, // 65535
        ]
    );
}

#[test]
fn pack_u32() {
    let mut writer = vec![];
    let vals = vec![65535, 65536, 4294967295];

    for v in vals {
        msgpack::pack_u32(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xce, 0x00, 0x00, 0xff, 0xff, // 65535
            0xce, 0x00, 0x01, 0x00, 0x00, // 65536
            0xce, 0xff, 0xff, 0xff, 0xff, // 4294967295
        ]
    );
}

#[test]
fn pack_u64() {
    let mut writer = vec![];
    let vals = vec![4294967295, 4294967296, 18446744073709551615];

    for v in vals {
        msgpack::pack_u64(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xcf, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, // 4294967295
            0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // 4294967296
            0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 18446744073709551615
        ]
    );
}

#[test]
fn pack_i8() {
    let mut writer = vec![];
    let vals = vec![-1, -32, -33, -128];

    for v in vals {
        msgpack::pack_i8(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd0, 0xff, // -1
            0xd0, 0xe0, // -32
            0xd0, 0xdf, // -33
            0xd0, 0x80, // -128
        ]
    );
}

#[test]
fn pack_i16() {
    let mut writer = vec![];
    let vals = vec![-128, -129, -32768];

    for v in vals {
        msgpack::pack_i16(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd1, 0xff, 0x80, // -128
            0xd1, 0xff, 0x7f, // -129
            0xd1, 0x80, 0x00, // -32768
        ]
    );
}

#[test]
fn pack_i32() {
    let mut writer = vec![];
    let vals = vec![-32768, -32769, -2147483648];

    for v in vals {
        msgpack::pack_i32(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd2, 0xff, 0xff, 0x80, 0x00, // -32768
            0xd2, 0xff, 0xff, 0x7f, 0xff, // -32769
            0xd2, 0x80, 0x00, 0x00, 0x00, // -2147483648
        ]
    );
}

#[test]
fn pack_i64() {
    let mut writer = vec![];
    let vals = vec![-2147483648, -2147483649, -9223372036854775808];

    for v in vals {
        msgpack::pack_i64(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd3, 0xff, 0xff, 0xff, 0xff, 0x80, 0x00, 0x00, 0x00, // -2147483648
            0xd3, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, // -2147483649
            0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // -9223372036854775808
        ]
    );
}

#[test]
fn pack_from_u8() {
    let mut writer = vec![];
    let vals = vec![1, 127, 128, 255];

    for v in vals {
        msgpack::pack_from_u8(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0x01, // 1
            0x7f, // 127
            0xcc, 0x80, // 128
            0xcc, 0xff, // 255
        ]
    );
}

#[test]
fn pack_from_u16() {
    let mut writer = vec![];
    let vals = vec![255, 256, 65535];

    for v in vals {
        msgpack::pack_from_u16(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xcc, 0xff, // 255
            0xcd, 0x01, 0x00, // 256
            0xcd, 0xff, 0xff, // 65535
        ]
    );
}

#[test]
fn pack_from_u32() {
    let mut writer = vec![];
    let vals = vec![65535, 65536, 4294967295];

    for v in vals {
        msgpack::pack_from_u32(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xcd, 0xff, 0xff, // 65535
            0xce, 0x00, 0x01, 0x00, 0x00, // 65536
            0xce, 0xff, 0xff, 0xff, 0xff, // 4294967295
        ]
    );
}

#[test]
fn pack_from_u64() {
    let mut writer = vec![];
    let vals = vec![4294967295, 4294967296, 18446744073709551615];

    for v in vals {
        msgpack::pack_from_u64(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xce, 0xff, 0xff, 0xff, 0xff, // 4294967295
            0xcf, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // 4294967296
            0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // 18446744073709551615
        ]
    );
}

#[test]
fn pack_from_i8() {
    let mut writer = vec![];
    let vals = vec![-1, -32, -33, -128];

    for v in vals {
        msgpack::pack_from_i8(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xff, // -1
            0xe0, // -32
            0xd0, 0xdf, // -33
            0xd0, 0x80, // -128
        ]
    );
}

#[test]
fn pack_from_i16() {
    let mut writer = vec![];
    let vals = vec![-129, -32768];

    for v in vals {
        msgpack::pack_from_i16(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd1, 0xff, 0x7f, // -129
            0xd1, 0x80, 0x00, // -32768
        ]
    );
}

#[test]
fn pack_from_i32() {
    let mut writer = vec![];
    let vals = vec![-32769, -2147483648];

    for v in vals {
        msgpack::pack_from_i32(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd2, 0xff, 0xff, 0x7f, 0xff, // -32769
            0xd2, 0x80, 0x00, 0x00, 0x00, // -2147483648
        ]
    );
}

#[test]
fn pack_from_i64() {
    let mut writer = vec![];
    let vals = vec![-2147483649, -9223372036854775808];

    for v in vals {
        msgpack::pack_i64(&mut writer, v).unwrap();
    }

    assert_eq!(
        writer,
        &[
            0xd3, 0xff, 0xff, 0xff, 0xff, 0x7f, 0xff, 0xff, 0xff, // -2147483649
            0xd3, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // -9223372036854775808
        ]
    );
}
fn gen_str(len: usize) -> String {
    let mut a = String::new();
    for _ in 0..len {
        a += "a";
    }
    a
}

#[test]
fn pack_fixstr() {
    let mut writer = vec![];
    msgpack::pack_str(&mut writer, "hello").unwrap();
    assert_eq!(writer, &[0xa5, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);

    let mut writer = vec![];
    msgpack::pack_str(&mut writer, &gen_str(31)).unwrap();
    assert_eq!(&writer[0..5], &[0xbf, 0x61, 0x61, 0x61, 0x61]);
}

#[test]
fn pack_str8() {
    let mut writer = vec![];
    msgpack::pack_str(&mut writer, &gen_str(32)).unwrap();
    assert_eq!(&writer[0..5], &[0xd9, 0x20, 0x61, 0x61, 0x61]);

    let mut writer = vec![];
    msgpack::pack_str(&mut writer, &gen_str(255)).unwrap();
    assert_eq!(&writer[0..5], &[0xd9, 0xff, 0x61, 0x61, 0x61]);
}

#[test]
fn pack_bin() {
    let mut writer = vec![];
    msgpack::pack_bin(&mut writer, gen_str(255).as_bytes()).unwrap();
    assert_eq!(&writer[0..5], &[0xc4, 0xff, 0x61, 0x61, 0x61]);

    let mut writer = vec![];
    msgpack::pack_bin(&mut writer, gen_str(256).as_bytes()).unwrap();
    assert_eq!(&writer[0..5], &[0xc5, 0x01, 0x00, 0x61, 0x61]);
}

#[test]
fn pack_fixary() {
    let mut writer = vec![];
    msgpack::pack_ary_header(&mut writer, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 1).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();

    assert_eq!(writer, &[0x92, 0x01, 0x02]);
}

#[test]
fn pack_ary16() {
    let mut writer = vec![];
    msgpack::pack_ary_header(&mut writer, (1 << 16) - 1).unwrap();
    for _ in 0..(1 << 16) {
        msgpack::pack_from_u8(&mut writer, 5 as u8).unwrap();
    }

    assert_eq!(writer[0..=2], [0xdc, 0xff, 0xff]);
    for i in 0..(1 << 16) {
        assert_eq!(writer[i + 3], 5 as u8);
    }
}

#[test]
fn pack_fixmap() {
    let mut writer = vec![];
    msgpack::pack_map_header(&mut writer, 1).unwrap();
    msgpack::pack_from_u8(&mut writer, 1).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();

    assert_eq!(writer, &[0x81, 0x01, 0x02]);
}

#[test]
fn pack_fixext() {
    let mut writer = vec![];
    msgpack::pack_ext_header(&mut writer, 1, 1).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();

    assert_eq!(writer, &[0xd4, 0x01, 0x02]);

    let mut writer = vec![];
    msgpack::pack_ext_header(&mut writer, 1, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    assert_eq!(writer, &[0xd5, 0x01, 0x02, 0x02]);

    let mut writer = vec![];
    msgpack::pack_ext_header(&mut writer, 1, 4).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    msgpack::pack_from_u8(&mut writer, 2).unwrap();
    assert_eq!(writer, &[0xd6, 0x01, 0x02, 0x02, 0x02, 0x02]);
}

#[test]
fn pack_timestamp() {
    let mut writer = vec![];
    msgpack::pack_timestamp32(&mut writer, 1).unwrap();
    assert_eq!(writer, &[0xd6, 0xff, 0x00, 0x00, 0x00, 0x01]);

    let mut writer = vec![];
    msgpack::pack_timestamp64(&mut writer, 1, 1).unwrap();
    assert_eq!(
        writer,
        &[0xd7, 0xff, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01]
    );

    let mut writer = vec![];
    msgpack::pack_timestamp96(&mut writer, 1, 1).unwrap();
    assert_eq!(
        writer,
        &[
            0xc7, 0x0c, 0xff, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01
        ]
    );
}
