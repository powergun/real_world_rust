// source
// rust std lib cookbook P/53
// a string is a kind of vector

#[test]
fn demo_string_from() {
    // creation
    // each char is a unicode scalar value
    // https://doc.rust-lang.org/std/primitive.char.html
    let mut s1 = String::from("map");
    s1.push(' ');
    s1.push('1');
    s1.push_str(" e1m1");
    assert_eq!("map 1 e1m1", s1);

    let _s2 = "".to_string();
    let _s3 = String::new();
    assert_eq!(_s2, _s3);
}

#[test]
fn demo_string_repeater() {
    assert_eq!("xxxxxx", String::from("xx").repeat(3));
}
