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

    // to create a vec of five ones!
    assert_eq!(vec![1; 5], vec![1, 1, 1, 1, 1])
    //              ^x ^num
}

#[test]
fn demo_specify_capacity() {
    let mut xs: Vec<i32> = Vec::with_capacity(10000);
    assert_eq!(10000, xs.capacity());
    xs.shrink_to_fit();
    assert_eq!(0, xs.capacity());
}

#[test]
fn demo_o1_swap_remove() {
    let mut xs = vec![1, 2, 3, 4];
    let x = xs.swap_remove(1);

    // swap(1) swaps [1] with the last element, 4
    // step one: [1, 4, 3, 2]
    // step two: [1, 4, 3]
    assert_eq!(vec![1, 4, 3], xs);
    assert_eq!(2, x);
}
