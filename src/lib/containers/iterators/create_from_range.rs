#[test]
fn demo_iterator_infinite() {
    let xs: Vec<_> = (1..) // 1 to infinity
        .map(|elem| elem + 1) // transform
        .filter(|&elem| elem % 5 == 0) // transform
        .take(7) // take the first seven values computed
        .collect(); // trigger evaluation

    assert_eq!(vec![5, 10, 15, 20, 25, 30, 35], xs);
}

// explained in 
// rust std lib cookbook P/59
#[test]
fn demo_create_from_range() {
    let it = 1..5;
    assert_eq!(1, it.filter(|&x| x == 3).count());

    // calling for .. in iter will consume the iterator
}