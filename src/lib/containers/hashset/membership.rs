#[allow(unused_imports)]
use std::collections::HashSet;

#[test]
fn demo_subset() {
    let s1: HashSet<_> = (1..10).collect();
    let s2: HashSet<_> = (2..3).collect();
    assert!(s2.is_subset(&s1));
}
