#[test]
fn demo_swap_elements() {
    let mut xs = vec![1, 2, 3, 4];

    xs.swap(1, 2);
    assert_eq!(vec![1, 3, 2, 4], xs);
}

#[test]
fn demo_remove_and_shift() {
    let mut xs = vec![1, 2, 3, 4];

    xs.remove(1);
    assert_eq!(vec![1, 3, 4], xs);
}

// retain those elements using a filter function, discard all
// the rest; note that the lambda takes a reference to T
#[test]
fn demo_filter_in_place() {
    let mut xs = vec![1, 2, 3, 4];

    xs.retain(|&x| x > 3);
    assert_eq!(vec![4], xs);
}

#[test]
fn demo_remove_duplicates() {
    let mut xs = vec![1, 1, 2, 2, 3, 3, 4, 4];

    xs.dedup();
    assert_eq!(vec![1, 2, 3, 4], xs);
}

#[test]
fn demo_consuming_iterator() {
    let mut xs = vec![1, 2, 3, 4];

    // recall that the range is exclusive
    // i.e. [.., 2)
    // therefore the remaining elements are [2], [3]
    xs.drain(..2).for_each(|_| {});
    assert_eq!(vec![3, 4], xs);
}
