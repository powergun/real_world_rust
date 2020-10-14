// source:
// rust std cookbook P/47

#[test]
fn demo_safe_access_element() {
    let xs = vec![1, 2, 3, 4];

    // see also get_mut() to mutate the element
    match xs.get(10) {
        Some(_) => panic!("shall not pass"),
        None => assert_eq!(4, xs.len()),
    };
    match xs.get(3) {
        // Some() wraps a reference to T
        Some(&x) => assert_eq!(4, x),
        None => assert_eq!(4, xs.len()),
    };

    // will explode
    // xs[12];
}
