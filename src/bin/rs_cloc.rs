use std::io::{stdin, Read, BufReader, BufRead};
use std::io;
use std::fs::{File, metadata};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::collections::HashSet;
use rayon::prelude::*;

fn read_from_stdin() -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    loop {
        let mut line = String::new();
        match stdin().read_line(&mut line) {
            Ok(usize) if usize > 0 => {
                lines.push(line.replacen("\n", "", 1));
            }
            _ => break,
        }
    }
    lines
}

struct Record {
    ext: String,
    loc: usize,
}

type MapT = HashMap<String, usize>;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

struct FileFilter {
    file_ext_black_list: HashSet<String>,
}

impl FileFilter {
    fn is_file(&self, filename: &String) -> bool {
        // source: https://stackoverflow.com/questions/30309100/how-to-check-if-a-given-path-is-a-file-or-directory
        match metadata(&filename) {
            Ok(md) => md.is_file(),
            _ => false,
        }
    }

    fn is_accepted_file_type(&self, filename: &String) -> bool {
        let ext = get_extension_from_filename(filename);
        !self.file_ext_black_list.contains(&ext)
    }

    fn accept(&self, filename: &String) -> bool {
        self.is_file(filename) && self.is_accepted_file_type(filename)
    }

    fn new() -> Self {
        let mut hs: HashSet<String> = HashSet::new();
        hs.insert("".to_string());
        hs.insert("gzip".to_string());
        hs.insert("o".to_string());
        hs.insert("cache".to_string());
        Self { file_ext_black_list: hs }
    }
}

fn get_extension_from_filename(filename: &str) -> String {
    match Path::new(filename).extension().and_then(OsStr::to_str) {
        Some(x) => x.to_string(),
        _ => String::new(),
    }
}

fn create_loc_record(filename: &String) -> io::Result<Record> {
    let ext = get_extension_from_filename(&filename);
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    Ok(Record { ext, loc: buf_reader.lines().count() })
}

fn update(mut r: Record, map: &mut MapT) -> () {
    if let Some(record) = map.get(&r.ext) {
        r.loc += record;
    }
    map.insert(r.ext, r.loc);
}

fn summarise(map: &MapT, max_num: usize) {
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x, y| y.1.cmp(&x.1));
    v.iter().take(max_num).for_each(|tu| {
        println!("{}: {}", tu.0, tu.1);
    });
}

fn main() {
    let lines = read_from_stdin();
    let mut map = MapT::new();
    let file_filter = FileFilter::new();
    let records = lines.par_iter()
        .filter_map(|line| {
            if file_filter.accept(&line) {
                match create_loc_record(line) {
                    Ok(record) => Some(record),
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<Record>>();
    records.into_iter().for_each(|record| update(record, &mut map));
    summarise(&map, 10);
}