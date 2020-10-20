// rust std lib cookbook P/137
extern crate serde_json;

#[allow(unused_imports)]
use serde_json::json;

#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::io::{self, BufRead};

#[test]
fn demo_dump_hash_map_to_json() {
    // untyped json
    let mut ht = HashMap::new();
    // ht.insert("map", "e1m1");

    // the json! macro lets us convert a value into its json
    // representation
    // how to import macro from crate, see:
    // https://doc.rust-lang.org/edition-guide/rust-2018/macros/macro-changes.html
    ht.insert("episode", json!(1));

    let blob = serde_json::to_string_pretty(&ht).expect("fail to serde");
    let expected_blob = r#"{
  "episode": 1
}"#;

    // NOTE, the order of the keys is not guaranteed!
    // (I was testing with two entries and the assertion below
    // would fail)
    assert_eq!(expected_blob, blob);
}
