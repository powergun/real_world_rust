// this is the first level of useless-ness.

#[test]
fn demo_immu_borrowing_of_primitive_type() {
    let x = 1;

    // can have as many immutable borrowers as I like
    let _x_ref1 = &x;
    let _x_ref2 = &x;

    // de-ref an immu ref to create another independent data
    // NOTE y is created-by-copying
    let y = *_x_ref1;
    assert_eq!(y, 1);
}

#[test]
fn demo_immu_borrowing_of_container_type() {
    let xs = vec![1, 2, 3];

    let _xs_ref1 = &xs;

    // CAN NOT do this
    // container type assignment will only transfer ownership
    // to the underlying block of memory instead of copying
    // this is equivalent to C++'s move assignment
    // let ys = *_xs_ref1;

    // I need to tell the compiler explicitly that I want to
    // clone the underlying block of memory
    let ys = _xs_ref1.clone();
    // assert_eq! uses by-reference parameters
    assert_eq!(ys, xs);
}

#[test]
fn demo_mut_borrowing_of_primitive_type() {
    let mut x = 0;

    let _x_ref1 = &x;

    let x_ref = &mut x;
    *x_ref += 1;

    // mut ref is no longer used, otherwise this is forbidden;
    // (even read/de-ref of x_ref is not allowed)
    let _x_ref2 = &x;
    assert_eq!(*_x_ref2, 1);
}
