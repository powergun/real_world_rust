// source
// rust std lib cookbook P/34

#[test]
fn demo_raw_string() {
    let s = r"c:\system";
    assert_eq!("c:\\system", s);
}
