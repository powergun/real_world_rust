
#[allow(unused_imports)]
use std::fs::{File, OpenOptions};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufReader, BufWriter, Lines, Write};

#[test]
fn demo_read_file_as_a_string() {
    let path = "/tmp/rw_rust_testdata/simple.txt";
    let read_file = |path: &str| -> io::Result<String> {
        // opens the file in read-only mode
        let file = File::open(path)?;

        // wrap the file in a BufReader to read in an efficient
        // way
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        Ok(content)
    };
    let content = read_file(path).expect("fail to read");
    assert!(content.len() > 4);
}
