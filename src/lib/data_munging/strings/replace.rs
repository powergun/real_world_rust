// mentioned in
// rust std lib cookbook P/56

#[test]
fn demo_replace_all_substrings() {
    let text = r"there is a heredoc";
    assert_eq!("th- is a h-doc", text.replace("ere", "-"));
}

// see also string.to_lowercase()
