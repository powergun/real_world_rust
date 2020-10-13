// source
// rust std cookbook P/48

#[test]
fn demo_search_element() {
    let xs = vec![1, 2, 3, 4];

    // must use reference
    assert!(xs.contains(&1));
}
