// C++ std::shared_ptr<>
use std::rc::Rc;

fn _demo_rc_creation() {
    let s = String::from("thereisacow");
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
