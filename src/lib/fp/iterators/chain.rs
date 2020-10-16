#[test]
pub fn test_chain_iterator() {
    // L431
    let o: Vec<i32> = (1..4).chain((1..4).map(|x| x * x)).collect();
    assert_eq!(vec![1, 2, 3, 1, 4, 9], o);
}
