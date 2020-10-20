// source
// rust std lib cookbook P/34

#[test]
fn demo_raw_string() {
    // read:
    // https://rahul-thakoor.github.io/rust-raw-string-literals/
    // like c++, r#"..."#, `#` is the delimiter
    let s = r#"c:\sys"tem"#;
    assert_eq!("c:\\sys\"tem", s);
}
