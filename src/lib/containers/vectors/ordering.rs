// source
// rust std cookbook P/48

#[test]
fn demo_sort_default() {
    let mut xs = vec![3, 1, 4, 1, 5];
    xs.sort();

    assert_eq!(vec![1, 1, 3, 4, 5], xs);
}

#[test]
fn demo_reverse_default() {
    let mut xs = vec![1, 2, 3, 4];
    xs.reverse();

    assert_eq!(vec![4, 3, 2, 1], xs);
}
