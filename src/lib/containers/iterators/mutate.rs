// mentioned in 
// rust std lib cookbook P/66
// see also:
// https://stackoverflow.com/questions/29322932/is-there-an-elegant-solution-to-modifying-a-structure-while-iterating

// there are two ways to modify the original elements:
// for x in &mut ctn
// ctn.iter_mut().for_each(...)
#[test]
fn demo_modify_elements() {
    let mut xs = vec![1, 2, 3];
    for x in &mut xs {
        *x += 1;
    }
    assert_eq!(vec![2, 3, 4], xs);

    xs.iter_mut().for_each(|x| {
        *x -= 1;
    });
    assert_eq!(vec![1, 2, 3], xs);

    // into_iter() 
    // consumes the items: takes ownership of them by moving them
    xs.into_iter().for_each(|_| {});
    // won't compile:
    // xs is consumed (the container itself)
    // assert_eq!(0, xs.len());
}

