// see also trim_left() and trim_right()
#[test]
fn demo_trim_leading_trailing_space() {
    let text = r"
there is a cow
";
    assert_eq!("there is a cow", text.trim());
}
