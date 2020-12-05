use std::io::{stdin, Read, BufReader, BufRead};
use std::io;
use std::fs::{File, metadata};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;

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

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn create_loc_record(filename: &String) -> io::Result<Record> {
    let ext = match get_extension_from_filename(&filename) {
        Some(x) => x.to_string(),
        _ => String::new(),
    };
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

fn accept(filename: &String) -> bool {
    // source: https://stackoverflow.com/questions/30309100/how-to-check-if-a-given-path-is-a-file-or-directory
    match metadata(&filename) {
        Ok(md) => md.is_file(),
        _ => false,
    }
}

fn summarise(map: &MapT, max_num: usize) {
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x,y| y.1.cmp(&x.1));
    v.iter().take(max_num).for_each(|tu| {
        println!("{}: {}", tu.0, tu.1);
    });
}

fn main() {
    let lines = read_from_stdin();
    let mut map = MapT::new();
    lines.iter()
        .filter(|line| accept(line))
        .for_each(|line| {
            if let Ok(record) = create_loc_record(line) {
                update(record, &mut map);
            }
        });
    summarise(&map, 10);
}