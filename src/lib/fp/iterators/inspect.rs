#[test]
pub fn test_inspect_iterator() {
    // L431
    // applies a function to all elements
    // inspect does not consume the element

    let mut outer_state = "".to_string();
    let o = (1..5)
        .inspect(|x| {
            // debug the element here
            outer_state = format!("{}{}", outer_state, x)
        })
        .fold(0, |acc, elem| acc + elem);
    assert_eq!("1234", outer_state);
    assert_eq!(10, o);
}
