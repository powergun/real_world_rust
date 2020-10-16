// stepping
// step each element by calling next()
// note the result is Optional()

// forwarding
// mentioned in:
// rust std lib cookbook P/60
#[test]
fn demo_forward_to_nth() {
    // dest position is in range
    {
        let mut it = "there".chars();
        match it.nth(3) {
            Some(ch) => assert_eq!('r', ch),
            _ => panic!("shall not fail"),
        }
    }
    // dest position is out of range
    {
        let mut it = "there".chars();
        match it.nth(30) {
            None => assert!(true),
            _ => panic!("shall not pass"),
        }
    }
}

#[test]
fn demo_last_element() {
    {
        let it = "there".chars();
        match it.last() {
            Some(ch) => assert_eq!('e', ch),
            _ => panic!("shall not fail"),
        }

        // won't compile; the iterator has been consumed
        // it.next();
    }
}
