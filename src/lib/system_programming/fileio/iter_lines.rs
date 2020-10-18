
// rust std lib cookbook P/91

use std::fs::{File};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufReader, Lines};

#[allow(dead_code)]
type FileIter = Lines<BufReader<File>>;

#[test]
fn demo_read_and_iter_lines() {
    
    // P/91
    // reading file line-by-line would inefficient when dealing
    // with a large file, which is why a BufReader actually
    // reads a large chunk of the file in one go and then returns
    // that segment line by line
    let iter_lines = |path: &str| -> io::Result<FileIter> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);

        // lines() returns an iterator over lines
        Ok(buf_reader.lines())
    };

    let iter = iter_lines("/tmp/rw_rust_testdata/simple.txt")
        .expect("fail to read");
    assert!(iter.count() > 1);
}
