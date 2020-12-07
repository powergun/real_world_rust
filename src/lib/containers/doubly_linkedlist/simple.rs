#[allow(unused_imports)]
use std::rc::{Weak, Rc};
#[allow(unused_imports)]
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
struct DlNode<T> {
    value: T,
    prev: Option<Weak<RefCell<DlNode<T>>>>,
    next: Option<Rc<RefCell<DlNode<T>>>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct DlList<T> {
    first: Option<Rc<RefCell<DlNode<T>>>>,
    last: Option<Weak<RefCell<DlNode<T>>>>,
}

#[allow(dead_code)]
impl<T> DlNode<T> {
    fn traverse(&self, f: fn(&T) -> ()) {
        f(&self.value);
        match self.next {
            Some(ref node) => node.borrow().traverse(f),
            _ => (),
        }
    }
}

#[allow(dead_code)]
impl<T> DlList<T> {
    fn empty() -> Self {
        DlList { first: None, last: None }
    }

    fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(curr_first) => {
                let new_first = Rc::new(RefCell::new(
                    DlNode {
                        value: data,
                        prev: None,
                        next: Some(curr_first.clone()),
                    }
                ));
                // prev is a weak ptr
                curr_first.borrow_mut().prev = Some(Rc::downgrade(&new_first));
                self.first = Some(new_first);
            }
            _ => {
                let new_first = Rc::new(RefCell::new(
                    DlNode {
                        value: data,
                        prev: None,
                        next: None,
                    }
                ));
                self.last = Some(Rc::downgrade(&new_first));
                self.first = Some(new_first);
            }
        }
    }

    fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(curr_last) => {
                let new_last = Rc::new(RefCell::new(
                    DlNode {
                        value: data,
                        prev: Some(curr_last.clone()),
                        next: None,
                    }
                ));
                let curr_last_strong = Weak::upgrade(&curr_last).unwrap();
                self.last = Some(Rc::downgrade(&new_last));
                curr_last_strong.borrow_mut().next = Some(new_last);
            },
            _ => {
                let new_last = Rc::new(RefCell::new(
                    DlNode {
                        value: data,
                        prev: None,
                        next: None,
                    }
                ));
                self.last = Some(Rc::downgrade(&new_last));
                self.first = Some(new_last);
            },
        }
    }

    fn traverse(&self, f: fn(&T) -> ()) {
        match self.first {
            // because traverse() takes ref-to-self, the match statement
            // must use ref-semantics (not value semantics)
            Some(ref node) => node.borrow().traverse(f),
            _ => (),
        }
    }
}

#[test]
fn demo_simple_dl_list_creation() {
    let mut dl = DlList::<i32>::empty();
    dl.push_front(1);
    dl.push_back(-1);
    dl.push_front(2);
    dl.push_back(-2);
    dl.push_front(3);
    dl.push_back(-3);

    dl.traverse(|x: &i32| {
        print!("{}", x);
    });

}
