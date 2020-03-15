// source
// https://doc.rust-lang.org/book/ch15-04-rc.html

use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    value: String,
}

impl Node {
    pub fn new(s: String) -> NodeType {
        Rc::new(RefCell::new(Node { value: s.clone() }))
    }
}

pub fn demo_take() {
    let n1 = Node::new("AA".to_string());
    {
        let _n2 = n1.clone(); // if n2 lives in the outer scope,
                              // it will cause panic() as it
                              // violates the borrowing rules
    }
    println!("{:?}", n1.borrow_mut());

    // try_unwrap()... gives back the RefCell object
    let v = Rc::try_unwrap(n1).ok().expect("///failed///").into_inner();
    println!("{:?}", v);
}

#[test]
fn demo_all() {
    demo_take();
}
