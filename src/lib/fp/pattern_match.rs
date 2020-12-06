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

#[test]
fn demo_match_range() {
    fn f(x: i32) -> String {
        match x {
            1..=99 => String::from("i"),
            144 => String::from("s"),
            188 => String::from("S"),
            _ => String::from(""),
        }
    }

    assert_eq!(f(1), "i");
    assert_eq!(f(100), "");
}

#[test]
fn demo_match_with_if_statement() {
    // this is probably borrowed from Scala

    fn f(x: i32) -> (char, i32) {
        match x {
            // val is essentially x
            val if (x % 3 == 0) => ('k', val),
            _ => ('\0', 0),
        }
    }
    assert_eq!(f(27).0, 'k');
}

#[test]
fn demo_match_tuple() {
    fn f(tpl: (i32, i32)) -> i32 {
        match tpl {
            (1, _) => 1_000_000,
            (2..=100, _) => 10,
            // note that if-statement must be placed outside
            // the tuple
            (_, x) if (x % 3 == 0) => x,
            _ => 0,
        }
    }

    assert_eq!(f((-1, 27)), 27);
}

#[test]
fn demo_ref_pattern() {
    // source:
    // https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;
    assert_eq!(ref_c1, ref_c2);
}