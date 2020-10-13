// strings and vectors are both heap-allocated
// they consist a pointer to the heap memory and a little metadata
// such as length

#[test]
fn demo_creation() {
    // specify type of element
    let mut elements: Vec<i32> = Vec::new();
    elements.push(1);
    assert_eq!(vec![1], elements);

    // can not infer type for T
    // let mut a = Vec::new();

    // turbo fish
    let xs = Vec::<i32>::new();
    assert_eq!(0, xs.len());

    // to create a vec of ten 1s!
    assert_eq!(vec![1; 5], vec![1, 1, 1, 1, 1])
}
