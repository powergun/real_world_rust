#[test]
pub fn test_map_iterator() {
    // L423
    // range is exclusive!
    // map() yields a thunk; fold evaluates this thunk
    let o = (1..4).map(|x| x * x).fold(0, |acc, elem| acc + elem);
    assert_eq!(o, 14);
}
