
// source
// rust std lib cookbook P/49

#[test]
fn demo_split_first() {
    let xs = vec![1, 2, 3, 4, 5];

    if let Some((first, xs_)) = xs.split_first() {
        assert_eq!(first, &1);
        assert_eq!(4, xs_.len());
    } else {
        panic!("shall not fail!");
    }
}
