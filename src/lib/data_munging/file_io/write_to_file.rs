// source
// https://doc.rust-lang.org/std/fs/struct.File.html

use std::fs::File;
use std::io::prelude::*;

pub fn demo(filename: String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(b"...")?;
    Ok(())
}

#[test]
fn demo_all() {
    demo("/var/tmp/test.txt".to_string()).expect("failed");

    // write to a non-existing file

    // thread 'main' panicked at 'failed: Os { code: 2, kind: NotFound,
    // message: "No such file or directory" }', src/libcore/result.rs:997:5
    // note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // demo("/vaer/wr23/423".to_string()).expect("failed");
}
