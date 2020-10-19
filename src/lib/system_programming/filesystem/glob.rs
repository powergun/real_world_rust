// rust std lib cookbook P/113

extern crate glob;
#[allow(unused_imports)]
use glob::{glob, glob_with, MatchOptions};

#[test]
fn demo_glob() {
    for entry in glob("./**/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(_path) => {
                // println!("{:?}", _path.display())
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // glob returns an iterator
    let it = glob("./**/*.rs").expect("Failed to read glob pattern");
    assert!(it.count() > 1);
}

#[test]
fn demo_glob_with() {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: true,
        ..Default::default()
    };
    let glob_it = glob_with("./**/*.rs", options).expect("Failed to read glob pattern");
    // assert!(it.count() > 1);

    // how to handle OsStr
    // https://doc.rust-lang.org/std/ffi/struct.OsStr.html
    use regex::Regex;
    let re = Regex::new(r"^[a-zA-Z]{4,6}.rs$").unwrap();
    let num = glob_it
        .filter_map(Result::ok)
        .filter(|path| match path.file_name() {
            None => false,
            Some(os_str) => re.is_match(os_str.to_str().unwrap_or("")),
        })
        .count();
    assert!(num > 1);
}
