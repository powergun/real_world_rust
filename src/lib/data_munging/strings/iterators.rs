// source
// rust std lib cookbook P/54

#[test]
fn demo_iterate_over_lines_in_string() {
    // how to convert a string to a vector of bytes
    // https://stackoverflow.com/questions/23850486/how-do-i-convert-a-string-into-a-vector-of-bytes-in-rust
    let text = String::from(
        r"
there is a cow,
there
is a
cow
",
    );
    // how to iterate over lines in a string
    // https://doc.rust-lang.org/std/primitive.str.html#method.lines
    let mut lines = text.lines();
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("there is a cow,"), lines.next());
}

#[test]
fn demo_take_1st_char_use_iter() {
    // next() returns an option value
    let xs: Vec<String> = vec!["e1m1".to_string(), "".to_string(), "ii".to_string()];
    let n = xs
        .iter()
        .filter(|x| {
            if let Some(_first_char) = x.chars().next() {
                false
            } else {
                true
            }
        })
        .count();
    // one string is empty, hence does not have the 1st char
    assert_eq!(n, 1);
}
