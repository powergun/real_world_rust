// tuple: a collection of various types
// tuple can be mutable in rust - elements can be updated;
// is indexed
// limited to 12 elements (recall that Scala allows for 25)

#[test]
fn demo_all() {
    let t: (i32, char) = (21, 'a');
    // tuple elements are accessed via dot
    println!("{} {}", t.0, t.1);
}

#[test]
fn decompose_array_to_variables() {
    let (v1, _) = ('a', 21);
    assert_eq!(v1, 'a');

    // rust doesn't seem to support ConsList-like head::tail
    // decomposition; there is no constructor-based
    // decomposition either (unlike Scala)
}

#[test]
fn update_elements() {
    let mut tpl = (21, 'a', "asd".to_string(), [0; 3]);
    assert_eq!(tpl.3, [0, 0, 0]);
    tpl.3[0] = 999;
    assert_eq!(tpl.3, [999, 0, 0]);
}