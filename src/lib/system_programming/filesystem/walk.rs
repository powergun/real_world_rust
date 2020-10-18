extern crate walkdir;
#[allow(unused_imports)]
use walkdir::{DirEntry, WalkDir};

#[test]
fn demo_walkdir() {
    for entry in WalkDir::new(".") {
        if let Ok(_entry) = entry {
            // println!("{}", _entry.path().display());
        }
    }

    // use walkdir iterator
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
    let is_dir = |entry: &DirEntry| -> bool {
        entry.file_type().is_dir()
    };
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
        .filter_map(|entry| {
            match entry.metadata() {
                Ok(meta) => Some(meta),
                _ => None,
            }
        })
        .any(|meta| meta.permissions().readonly());

    // calculate the total size
    let sz = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| {
            match entry.metadata() {
                Ok(meta) => Some(meta),
                _ => None,
            }
        })
        .filter(|meta| meta.is_file())
        .fold(0, |acc, m| acc + m.len());
    assert!(sz > 1);
}
