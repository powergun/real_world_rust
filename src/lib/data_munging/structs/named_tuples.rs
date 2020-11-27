#[allow(dead_code)]
struct Map(String, i32);

#[test]
fn demo_named_tuple() {
    let m = Map("e1m1".to_string(), 12);
    assert_eq!(m.1, 12);
}
