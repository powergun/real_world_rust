extern crate walkdir;
use walkdir::{DirEntry, WalkDir};
use std::path::Path;

fn is_ignored(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            s.starts_with(".") || s == "target" || s == "testdata"
        })
        .unwrap_or(false)
}

#[allow(dead_code)]
fn is_git(entry: &DirEntry) -> bool {
    let n = entry.file_name().to_str().expect("");
    n == ".git"
}

#[allow(dead_code)]
fn is_source_tree(entry: &DirEntry) -> bool {
    let n = entry.file_name().to_str().expect("");
    n == "src"
}

pub fn do_walk(entry_point: &str) {
    let pth = Path::new(entry_point).canonicalize().expect("fail to canonicalize()");
    WalkDir::new(pth)
        .into_iter()
        .filter_entry(|entry| !is_ignored(entry))
        .filter_map(Result::ok)
        .for_each(|entry| {
            process_file(entry.path());
        });
}

// see:
// https://doc.rust-lang.org/std/path/struct.Path.html
fn process_file(path: &Path) {
    println!("{}", path.display());
}
