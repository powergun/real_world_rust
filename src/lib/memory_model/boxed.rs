fn _demo_box() {
    // like C's malloc() and C++'s smart pointer
    // R/W access
    // RAII idiom - only used in one place, resource is deallocated
    // once it goes out of scope
    let _boxed = Box::new("data");
}

#[test]
fn demo_all() {
    _demo_box();
}

#[test]
fn demo_take_ref_return_ref() {
    fn trim_left_char(s: &str) -> &str {
        match s.get(1..s.len()) {
            Some(ss) => ss,
            _ => "",
        }
    }
    {
        let trimmed = trim_left_char("e1m1");
        assert_eq!(trimmed, "1m1");
    }
    {
        let original = "e1m1".to_string();
        // this creates a copy of `original`
        let trimmed = trim_left_char(&original);
        assert_eq!(trimmed, "1m1");
        assert_eq!(original, "e1m1");
    }
}

// Boxed type is useful to model recursive type (where the size
// is unknown); value-semantic and ref-semantic is not applicable
// due to 1. recursive size calculation, 2. life time management
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum LinkedList<T> {
    Empty,
    Cons(T, Box<LinkedList<T>>),
}

impl<T> LinkedList<T> {
    #[allow(dead_code)]
    fn empty() -> Self {
        Self::Empty
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        // source:
        // pattern match with enum type
        // https://doc.rust-lang.org/book/ch06-02-match.html
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn singleton(x: T) -> Self {
        Self::Cons(x, Box::new(Self::Empty))
    }

    #[allow(dead_code)]
    fn head(&self) -> Option<&T> {
        ////////////////////////////////////////////////////////
        // this method does not consume `self` (it uses an imu ref)
        match self {
            Self::Empty => None,
            Self::Cons(hd, _) => Some(hd),
        }
    }

    #[allow(dead_code)]
    fn push_left(self, x: T) -> Self {
        ////////////////////////////////////////////////////////
        // this method consumes `self` (it uses value semantics)
        Self::Cons(x, Box::new(self))
    }

    #[allow(dead_code)]
    fn push(&mut self, x: T) {
        ////////////////////////////////////////////////////////
        // this method mutates self
        // Boxed type implements the Deref trait, meaning caller
        // can call members of T from the boxed type directly
        match self {
            Self::Empty => *self = Self::singleton(x),
            Self::Cons(_, tl) => tl.push(x),
        }
    }
}

#[test]
fn test_linked_list() {
    let empty_list: LinkedList<i32> = LinkedList::empty();
    assert_eq!(empty_list.is_empty(), true);
    assert_eq!(empty_list.head(), None);

    let sin_list: LinkedList<i32> = LinkedList::singleton(12);
    assert_eq!(sin_list.is_empty(), false);
    assert_eq!(sin_list.head().unwrap(), &12);

    let list2 = sin_list.push_left(1);
    assert_eq!(list2.head().unwrap(), &1);
    // sin_list has been consumed
    // assert_eq!(sin_list.head().unwrap(), &12);

    {
        let mut ll: LinkedList<i32> = LinkedList::empty();
        ll.push(1);
        assert_eq!(ll.head().unwrap(), &1);
    }
}
