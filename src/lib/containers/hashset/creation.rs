// rust std lib cookbook P/76

#[allow(unused_imports)]
use std::collections::HashSet;

#[test]
fn demo_create_set_from_range() {
    let mut xs: HashSet<_> = (1..10).collect();

    assert!(xs.contains(&3));
    assert!(xs.remove(&3));
}
