// C++ std::shared_ptr<>
#[allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;

fn _demo_rc_creation() {
    let s = String::from("there is ac ow");
    {
        let ptr1 = Rc::new(s);
        // I originally used this to demo the point
        // let mut ptr1 = Rc::new(s);  but rust compiler and IDE warns that
        // ptr1 does not need to be mutable
        // the content of s is now owned by ptr1
        // put the content is immut due to
        // multiple borrowing rule
        let ptr2 = ptr1.clone(); // increment ptr1 counter
        println!("{}, {}", ptr1, ptr2);
    }
}

#[test]
fn demo_all() {
    _demo_rc_creation();
}

////////////////////////////////////////////////////////////////
// re-implementing the ConsList (LinkedList) example in boxed.rs`
// this time, allows for mutability in the internal data model
//
// Rc has to work with RefCell to model mutability
//
// i.e. this won't work
// let a = Rc::new(1)
// *a += 2;
// error: can not borrow rc as mutable
// see the demo below

#[test]
fn demo_rc_refcell_mutability_modelling() {
    let _a = Rc::new(1);
    let b = Rc::new(RefCell::new(1));
    assert_eq!(*(b.borrow()), 1);

    // mutate it
    *(b.borrow_mut()) = 12;
    assert_eq!(*(b.borrow()), 12);
}

type TailT<T> = Rc<RefCell<ConsList<T>>>;

#[allow(dead_code)]
enum ConsList<T> {
    Empty,
    Cons(T, TailT<T>),
}

impl<T> ConsList<T> {
    #[allow(dead_code)]
    fn empty() -> Self {
        ConsList::Empty
    }

    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn singleton(x: T) -> Self {
        ConsList::Cons(x, TailT::new(RefCell::new(ConsList::Empty)))
    }

    #[allow(dead_code)]
    fn push_left(self, x: T) -> Self {
        match self {
            Self::Empty => Self::singleton(x),
            _ => Self::Cons(x, Rc::new(RefCell::new(self))),
        }
    }

    #[allow(dead_code)]
    fn head(&self) -> Option<&T> {
        match self {
            Self::Empty => None,
            Self::Cons(hd, _) => Some(hd),
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: T) {
        match self {
            Self::Empty => *self = Self::singleton(x),
            Self::Cons(_, tl) => tl.borrow_mut().push(x),
        }
    }
}

#[test]
fn test_cons_list() {
    let l0: ConsList<i32> = ConsList::empty();
    assert_eq!(l0.is_empty(), true);

    let l1: ConsList<i32> = ConsList::singleton(1);
    assert_eq!(l1.is_empty(), false);
    assert_eq!(l1.head().unwrap(), &1);

    let mut l2: ConsList<i32> = l1.push_left(2);
    assert_eq!(l2.head().unwrap(), &2);

    l2.push(4);
    assert_eq!(l2.head().unwrap(), &2);
}
