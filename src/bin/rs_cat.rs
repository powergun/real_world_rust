// FP in rust L937
// usage:

// cargo build && cd target/debug
// cat <a txt file> | ./cat -
// ./cat <a txt file>

// or using cargo run
// cargo run --bin cat <(echo asd)

// use stdin
// echo asd | cargo run --bin cat
// or
// cat <<SRC | cargo run --bin cat
// > there
// > is a
// > silence
// > SRC

// Note: like the Linux cat, file takes higher precedence to
// stdin; if one or more files are given, stdin is ignored

use std::io::{stdin, Read};

fn read_from_stdin() -> String {
    let mut lines = String::new();
    match stdin().read_to_string(&mut lines) {
        Ok(_) => lines,
        _ => String::new(),
    }
}

fn main() {
    let buffer = match std::env::args().nth(1) {
        Some(ref fp) if *fp == "-".to_string() => {
            let mut buffer = String::new();
            std::io::stdin()
                .read_to_string(&mut buffer)
                .expect("read_to_string failed");
            buffer
        }
        None => read_from_stdin(),
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
