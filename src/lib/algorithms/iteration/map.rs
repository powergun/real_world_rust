#[test]
fn demo_iterator_map() {
    // recall map {} in perl and ruby

    let nums = vec![3, 1, 4, 1, 5, 9];
    let operand = 1;
    let xs: Vec<_> = nums.iter()
        .map(|x| x + operand)
        .collect();
    // map() is a closure; it can use the variables in the
    // calling scope; same applies to filter();
    // there are different accessing mode: copy and move
    assert_eq!(xs, vec![4, 2, 5, 2, 6, 10]);

    // the original vector is unmodified, of course
    assert_eq!(nums, vec![3, 1, 4, 1, 5, 9]);
}
