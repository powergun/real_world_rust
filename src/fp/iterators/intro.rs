#[test]
pub fn test_map_iterator() {
    // L423
    // range is exclusive!
    // map() yields a thunk; fold evaluates this thunk
    let o = (1..4).map(|x| x * x).fold(0, |acc, elem| acc + elem);
    assert_eq!(o, 14);
}

#[test]
pub fn test_chain_iterator() {
    // L431
    let o: Vec<i32> = (1..4).chain((1..4).map(|x| x * x)).collect();
    assert_eq!(vec![1, 2, 3, 1, 4, 9], o);
}

#[test]
pub fn test_zip_iterator() {
    // L431
    // respect the length of the shortest iterator
    // iter.zip(iter').zip(iter'') yields {((x, x'), x''), ((x, x'), x''), ...}
    let o = (1..4)
        .zip(1..10)
        .zip(1..3)
        .fold(vec![0, 0, 0], |acc, elem| {
            vec![acc[0] + (elem.0).0, acc[1] + (elem.0).1, acc[2] + elem.1]
        });
    assert_eq!(vec![3, 3, 3], o);
}

#[test]
pub fn test_enumerate_iterator() {
    // L431
    let o: Vec<usize> = (1..4).enumerate().map(|pair| pair.0).collect();
    assert_eq!(vec![0, 1, 2], o);
}

#[test]
pub fn test_inspect_iterator() {
    // L431
    // applies a function to all elements (not mutating)
    let o = (1..5)
        .inspect(|x| print!("//{:?}", x))
        .fold(0, |acc, elem| acc + elem);
    assert_eq!(10, o);
}

#[test]
pub fn test_loop_call_iterator() {
    // L454
    let it = (1..5).map(|x| x).inspect(|x| print!("++{:?}", x));
    let mut o: i32 = 0;
    for elem in it {
        o += elem;
    }
    assert_eq!(10, o);
}
