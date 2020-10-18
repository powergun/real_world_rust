// rust std lib cookbook P/91

#[allow(unused_imports)]
use std::fs::{File, OpenOptions};
#[allow(unused_imports)]
use std::io::prelude::*;
#[allow(unused_imports)]
use std::io::{self, BufWriter};

// File::create() creates a new file if it doesn't exist,
// otherwise it truncates the file.
// overwriting a file is called "truncating"
#[allow(dead_code)]
fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    // create() opens a file with the standard options
    // to create, write and truncate a file
    // recall Go's create() function

    let file = File::create(path)?;

    // wrap the file in a BufWriter
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

#[test]
fn demo_write_string_to_file() {
    let text = r"
there is a cow
";
    write_to_file("/tmp/o", text).expect("fail to write");
}

#[test]
fn demo_append_and_read() {
    let append_to = |path: &str, content: &str| -> io::Result<()> {
        let file = OpenOptions::new().read(true).append(true).open(path)?;

        // rust std lib cookbook P/91
        // we can read and write on the same file handle - we
        // pass a reference to the file instead of moving it,
        // as the buffers would otherwise consume the handle,
        // making sharing impossible
        // (see write_to_file, which consumes the file handle)
        let mut buf_writer = BufWriter::new(&file);
        buf_writer.write_all(content.as_bytes())?;
        Ok(())
    };

    write_to_file("/tmp/o", "map name\n").expect("fail to write");
    append_to("/tmp/o", "creature name: \n").expect("fail to write");
}
