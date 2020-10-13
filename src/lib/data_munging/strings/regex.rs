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

#[test]
fn demo_use_case_flag() {
    use regex::Regex;
    let re = Regex::new(r"(?i)^id\w{3,6}$").unwrap();

    assert!(re.is_match("IDDQD"));
}

#[test]
fn demo_use_regex_builder() {
    use regex::RegexBuilder;
    let re = RegexBuilder::new(r"^id\w{3,6}$")
        .case_insensitive(true)
        .build()
        .unwrap();

    assert!(re.is_match("IDDQD"));
}
