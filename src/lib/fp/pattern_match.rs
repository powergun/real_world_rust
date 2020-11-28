#[allow(unused_imports)]
use std::env::args;

#[test]
fn single_case_multiple_values() {
    fn f() -> Result<i32, i32> {
        match args().count() {
            0 | 1 | 2 | 3 | 4 | 5 => Err(-1),
            20 | 30 | 40 => Ok(1),
            _ => Ok(1000),
        }
    }
    assert_eq!(f(), Err(-1));
}

// how to pattern match enum (used as a sum type)
// https://doc.rust-lang.org/book/ch06-02-match.html
// see
// memory management section, boxed.rs for an implementation of
// linked list that uses the boxed type
