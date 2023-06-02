use std::ffi::CString;
use node_buffer::{Buffer, StringEncoding};

fn main() {
    let mut buffer = Buffer::builder()
        .fill_text(CString::new("hello world").unwrap(), StringEncoding::Utf8)
        .build();

    println!("{}", buffer.to_string(Some(StringEncoding::Hex), None, None));
// Prints: 68656c6c6f20776f726c64
    println!("{}", buffer.to_string(Some(StringEncoding::Base64), None, None));
// Prints: aGVsbG8gd29ybGQ=

    println!("{}", Buffer::builder().fill_text(CString::new("fhqwhgads").unwrap(), StringEncoding::Utf8).build());
// Prints: <Buffer 66 68 71 77 68 67 61 64 73>
    println!("{}", Buffer::builder().fill_text(CString::new("fhqwhgads").unwrap(), StringEncoding::Utf16le).build());
// Prints: <Buffer 66 00 68 00 71 00 77 00 68 00 67 00 61 00 64 00 73 00>


    let buffer1: &[u8] = &[1, 2, 3];
    let buffer2: &[u8] = &[4, 5, 6];
    let buffer3: &[u8] = &[7, 8, 9];


    let buffer = Buffer::concat(&[buffer1, buffer2, buffer3], None);

    println!("data {}", buffer);

    let buffer = Buffer::builder()
        .size(11)
        .fill_text(CString::new("aGVsbG8gd29ybGQ=").unwrap(), StringEncoding::Base64)
        .build();

    println!("{}", buffer);

    let mut buffer = Buffer::builder()
        .size(26)
        .build();

    let buf = buffer.buffer_mut();

    for i in 0..26 as usize {
        buf[i] = (i + 97) as u8;
    }


    println!("{}", buffer.to_string(Some(StringEncoding::Utf8), None, None));
// Prints: abcdefghijklmnopqrstuvwxyz
    println!("{}", buffer.to_string(Some(StringEncoding::Utf8), Some(0), Some(5)));
// Prints: abcde


    let buffer = Buffer::from_slice(&[0x12, 0x34, 0x56, 0x78]);

    println!("{:x}", buffer.read_uint32le(Some(0)));
// Prints: 78563412
    // println!("{:x}",buffer.read_uint32le(Some(1)).unwrap());
// Throws ERR_OUT_OF_RANGE.


    let mut buffer = Buffer::builder()
        .size(8)
        .build();

    buffer.write_big_int64be(0x0102030405060708, Some(0));

    println!("{:}", buffer);


    let mut buffer = Buffer::builder()
        .size(8)
        .build();

    buffer.write_big_int64le(0x0102030405060708, Some(0));

    println!("{}", buffer);


    let mut buffer = Buffer::builder()
        .size(8)
        .build();

    buffer.write_big_uint64be(0xdecafafecacefade, Some(0));

    println!("{}", buffer);


    let mut buffer = Buffer::builder()
        .size(8)
        .build();

    buffer.write_big_uint64le(0xdecafafecacefade, Some(0));

    println!("{}", buffer);


    let mut buffer = Buffer::builder()
        .size(4)
        .build();

    buffer.write_uint8(0x3, Some(0));
    buffer.write_uint8(0x4, Some(1));
    buffer.write_uint8(0x23, Some(2));
    buffer.write_uint8(0x42, Some(3));

    println!("{}", buffer);
// Prints: <Buffer 03 04 23 42>


    let mut buffer = Buffer::builder()
        .size(2)
        .build();

    buffer.write_int16be(0x0102, None);

    println!("{}", buffer);
// Prints: <Buffer 01 02>


    let mut buffer = Buffer::builder()
        .size(4)
        .build();
    buffer.write_uint16be(0xdead, Some(0));
    buffer.write_uint16be(0xbeef, Some(2));

    println!("{}", buffer);
// Prints: <Buffer de ad be ef>
}
