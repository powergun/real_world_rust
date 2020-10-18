// inspired by:
// rust std lib cookbook P/41

// source:
// https://stackoverflow.com/questions/41069865/how-to-create-an-in-memory-object-that-can-be-used-as-a-reader-writer-or-seek/41069910
//

extern crate byteorder;

#[allow(unused_imports)]
use std::io::{Read, Write};

#[test]
fn demo_vec_as_file() {
    // Create fake "file"
    let mut file = Vec::new();

    // Write into the "file"
    file.write_all(&[1, 2, 3, 4, 5]).unwrap();

    // Read the "file's" contents into a new vector
    let mut out = Vec::new();
    let mut c = file.as_slice();
    c.read_to_end(&mut out).unwrap();

    println!("{:?}", out);
}

#[test]
fn demo_read_from_bytes() {
    // source
    // https://doc.rust-lang.org/std/io/trait.Read.html
    let mut bs = r"
there 
is a 
cow    
"
    .as_bytes();
    let mut buffer = [0; 255];
    bs.read(&mut buffer).unwrap();
}

// rust std lib cookbook P/94
#[test]
fn demo_create_buffer_by_cursor() {
    use std::io::Cursor;
    // brings read_u8() method to the reader
    // see also read_u32 and write_xx
    use byteorder::{ReadBytesExt, WriteBytesExt};

    let mut raw_bytes = vec![1, 2, 3, 4, 5, 6];

    // wrap a binary collection in cursor to provide seek
    // functionality
    let mut buff = Cursor::new(&mut raw_bytes);
    assert_eq!(1, buff.read_u8().expect("1st byte"));
    // reading advances the internal position, so now we read
    // the second raw byte
    assert_eq!(2, buff.read_u8().expect("2nd byte"));

    buff.write_u8(0x78).expect("3rd byte");
    // the 3rd byte is modified
    assert_eq!(vec![1, 2, 0x78, 4, 5, 6], raw_bytes);
}
