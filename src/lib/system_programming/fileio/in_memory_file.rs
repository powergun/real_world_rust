// inspired by:
// rust std lib cookbook P/41

// source:
// https://stackoverflow.com/questions/41069865/how-to-create-an-in-memory-object-that-can-be-used-as-a-reader-writer-or-seek/41069910
//

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
