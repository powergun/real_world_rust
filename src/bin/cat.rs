// FP in rust L937
// usage:
// cargo build && cd target/debug
// cat <a txt file> | ./cat -
// ./cat <a txt file>

use std::io::Read;

fn main() {
    let buffer = match std::env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
        None => {
            panic!("error: expect filename(s) or '-'");
        }
        Some(fp) => {
            let mut buffer = String::new();
            std::fs::File::open(fp)
                .expect("File::open failed")
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
    };
    for (_, l) in buffer.lines().enumerate() {
        println!("{}", l)
    }
}
