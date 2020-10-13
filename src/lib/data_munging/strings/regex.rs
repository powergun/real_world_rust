// source
// rust std lib cookbook P/32
// see also:
// https://docs.rs/regex/1.4.0/regex/

#[test]
fn demo_find_date() {
    use regex::Regex;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    assert!(re.is_match("2014-01-01"));
}
