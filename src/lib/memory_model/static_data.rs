#[allow(dead_code)]
static N: i32 = 1;

////////////////////////////////////////////////////////////////
// mutable static data is almost banned (requires unsafe)

#[test]
fn demo_use_static_data() {
    fn getter() -> &'static i32 {
        &N
    }
    assert_eq!(getter(), &1);
}

#[test]
fn demo_use_static_str() {
    fn getter() -> &'static str {
        "there is a silence"
    }
    assert_eq!(getter(), "there is a silence");
}
