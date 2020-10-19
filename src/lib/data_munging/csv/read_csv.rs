#[allow(unused_imports)]
use std::fs::{File};
#[allow(unused_imports)]
use std::io::{self, BufReader};

// source: csv crate tutorial
// https://docs.rs/csv/1.1.3/csv/tutorial/index.html
// good explanation on the basic error handling
// also Serde is great

#[test]
fn demo_read_csv() {
    // Create a CSV parser that reads data from file
    // let file = File::open("/tmp/rw_rust_testdata/simple.csv").expect("fail to open file");
    // let buf_reader = BufReader::new(file);
    // let mut rdr = csv::Reader::from_reader(buf_reader);

    let mut rdr = csv::Reader::from_path("/tmp/rw_rust_testdata/simple.csv").expect("fail to open file");

    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        // println!("{:?}", record);
        assert!(record.len() > 1);
    }
}
