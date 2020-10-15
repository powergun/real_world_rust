// source
// rust std lib P/55

#[test]
fn demo_substrings_by_pattern() {
    let text = "There is a Cow";
    assert_eq!(2, text.matches("e").count());

    assert_eq!(2, text.matches(char::is_uppercase).count());
}

// see also: ends_with()
#[test]
fn demo_starts_with() {
    let text = "There is a Cow";
    assert!(text.starts_with("There"));
    assert!(text.starts_with(char::is_uppercase));
}
