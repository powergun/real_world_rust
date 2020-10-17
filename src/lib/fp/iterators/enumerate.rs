// no C-style for loop!!!

#[test]
pub fn test_enumerate_iterator() {
    // L431
    let o: Vec<usize> = (1..4).enumerate().map(|pair| pair.0).collect();
    assert_eq!(vec![0, 1, 2], o);

}
