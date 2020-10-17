// rust std lib cookbook
// P/62

// P/63
#[test]
fn demo_filter_map() {
    let xs: Vec<_> = (2..10)
        .filter_map(|x| {
            for y in 2..(x / 2 + 1) {
                if y * y == x {
                    return Some(x);
                }
            }
            None
        })
        .collect();
    assert_eq!(vec![4, 9], xs);
}
