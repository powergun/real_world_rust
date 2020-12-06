#[test]
fn demo_iterator_filter() {
    // recall grep {} in perl and ruby

    let nums = vec![3, 1, 4, 1, 5, 9];
    let larger_then_three = nums
        .into_iter()
        .filter(|&x| x > 3)
        .collect::<Vec<i32>>();
    assert_eq!(larger_then_three, vec![4, 5, 9]);

    // collect the result in a vec
    // Vec<_> don't care about the type
    let items: Vec<_> = (1..10).filter(|&x| x > 3).collect();
    assert_eq!(items, vec![4, 5, 6, 7, 8, 9]);
}

#[test]
fn demo_iteration_using_range() {
    // 10..1 produces an empty range
    for _ in 10..1 {
        println!("asd");
    }
    assert!((10..1).collect::<Vec<_>>().is_empty());
}