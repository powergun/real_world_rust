use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[allow(unused_imports)]
use super::replace_in_file::replace_in_file;

pub fn read_file_ok(file_path: &str) -> Result<(), std::io::Error> {
    let mut src = File::open(&Path::new(&file_path))?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    Ok(())
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut src = File::open(&Path::new(&file_path))?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    Ok(data)
}

#[test]
fn test_read_file_return_ok() {
    let _ = read_file_ok("testdata/search_replace.txt");
}

#[test]
fn test_read_file_return_string() {
    let result = read_file("testdata/search_replace.txt");
    match result {
        Ok(v) => assert_eq!("there is a cow", v),
        Err(_) => panic!("failed!"),
    }

    let _ = replace_in_file("testdata/search_replace.txt", "cow", "COW");
    let result2 = read_file("testdata/search_replace.txt");
    match result2 {
        Ok(v) => assert_eq!("there is a COW", v),
        Err(_) => panic!("failed!"),
    }

    let _ = replace_in_file("testdata/search_replace.txt", "COW", "cow");
    let result3 = read_file("testdata/search_replace.txt");
    match result3 {
        Ok(v) => assert_eq!("there is a cow", v),
        Err(_) => panic!("failed!"),
    }
}
