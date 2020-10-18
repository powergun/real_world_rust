// rust std lib cookbook P/110

extern crate walkdir;
#[allow(unused_imports)]
use walkdir::{DirEntry, WalkDir};

#[test]
fn demo_walkdir() {
    // when will the Result be a failure:
    // - os prohibit you from reading
    // - a symbolic link (without enabling follow_links(true))
    // - or you point back to a parent dir, leading to cyclic loop
    for entry in WalkDir::new(".") {
        if let Ok(_entry) = entry {
            // Path is a struct that offers functionality
            // extension, parent etc.
            // recall boost's file path type
            // display() will produce a string

            // println!("{}", _entry.path().display());
        }
    }

    // use walkdir as an iterator, which has special methods
    // no other iterators have;
    // filter_entry() is an optimization over normal FP filter
    // in that it gets called during the traversal;
    // when its predicate returns false on a directory, the
    // walker will not go into that directory at all
    let is_hidden = |entry: &DirEntry| -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with("."))
            .unwrap_or(false)
    };
    WalkDir::new("/var/tmp")
        .into_iter() // consume the iterator
        .filter_entry(|entry| !is_hidden(entry)) // exclude hidden
        .filter_map(Result::ok)
        .for_each(|entry| {
            // convert the name returned by the os into a rust
            // string;
            // if there are any non-utf8 symbols in it, replace
            // them with placeholder characters
            let _name = entry.file_name().to_string_lossy();
            // println!("{}", _name);
        });

    // more creative use of the walkdir iterator:
    // find sub directories
    let is_dir = |entry: &DirEntry| -> bool { entry.file_type().is_dir() };
    WalkDir::new("..")
        .into_iter()
        .filter_entry(|entry| is_dir(entry)) // dirs only
        .filter_map(Result::ok)
        .for_each(|entry| {
            let _name = entry.file_name().to_string_lossy();
            // println!("{}", _name);
        });

    // use more metadata
    WalkDir::new("..")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.metadata() {
            Ok(meta) => Some(meta),
            _ => None,
        })
        .any(|meta| meta.permissions().readonly());

    // calculate the total size
    let sz = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| match entry.metadata() {
            Ok(meta) => Some(meta),
            _ => None,
        })
        .filter(|meta| meta.is_file())
        .fold(0, |acc, m| acc + m.len());
    assert!(sz > 1);
}
