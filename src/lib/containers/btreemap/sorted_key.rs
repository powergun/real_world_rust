#[allow(unused_imports)]
use std::collections::BTreeMap;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::prelude::*;

#[test]
fn demo_read_records_from_file_to_btree_map() {
    // btree map keeps the keys in sorted order
    // Note how the records (each line in /etc/passwd) are
    // "parsed" to stored into the accounts map

    let mut accounts: BTreeMap<String, String> = BTreeMap::new();
    {
        let mut file = File::open("/etc/passwd").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).expect("fail to read");
        for s in data.split("\n") {
            let v: Vec<&str> = s.split(':').collect();
            accounts.insert(
                v[0].to_string(),
                match v.get(5) {
                    Some(refs) => refs.to_string(),
                    _ => String::new(),
                },
            );
        }
    }
    let num_x = accounts
        .into_iter()
        .filter(|(k, v)| k.is_empty() || v.is_empty())
        .count();
    assert_eq!(num_x < 2, true);
}
