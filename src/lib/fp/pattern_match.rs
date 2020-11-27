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
