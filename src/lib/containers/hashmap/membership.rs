// rust std lib cookbook P/71

#[allow(unused_imports)]
use std::collections::HashMap;

#[test]
fn demo_key_exists() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });

    assert!(tb.contains_key("map"));
    assert!(!tb.contains_key("weapon"));
}

#[test]
fn demo_access_value() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });

    match tb.get("prop") {
        Some(&x) => assert_eq!(4, x),
        _ => panic!("shall not fail"),
    };

    match tb.get("asd") {
        None => assert!(true),
        _ => panic!("shall not pass"),
    };
}

#[test]
fn demo_overwrite_value() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });

    // insert() returns the old value wrapped in Optional
    // note that the value is not a ref - it has no relation
    // to the hash map
    match tb.insert("prop".to_string(), 111) {
        Some(x) => assert_eq!(4, x),
        _ => panic!("shall not fail"),
    };

    match tb.get("prop") {
        Some(&x) => assert_eq!(111, x),
        _ => panic!("shall not fail"),
    };
}

#[test]
fn demo_remove_value() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });

    // remove() returns the old value wrapped in Optional
    // note that the value is not a ref - it has no relation
    // to the hash map
    match tb.remove("prop") {
        Some(x) => assert_eq!(4, x),
        _ => panic!("shall not fail"),
    };
}

// P/73
#[test]
fn demo_or_insert() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });
    tb.entry("weapon".to_string()).or_insert(6);
    assert_eq!(4, tb.len());
}
