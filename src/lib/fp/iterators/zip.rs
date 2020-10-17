#[test]
pub fn test_zip_iterator() {
    // L431
    // respect the length of the shortest iterator
    // iter.zip(iter').zip(iter'') yields {((x, x'), x''), ((x, x'), x''), ...}
    let o = (1..4) // (1, 1, 1), (2, 2, 2), (3, 3, 3)
        .zip(1..10)
        .zip(1..3)
        .fold(vec![0, 0, 0], |acc, elem| {
            vec![acc[0] + (elem.0).0, acc[1] + (elem.0).1, acc[2] + elem.1]
        });
    assert_eq!(vec![3, 3, 3], o);
}
