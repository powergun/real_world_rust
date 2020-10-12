// hands on algorithms and data structures with rust P60

#[allow(unused_imports)]
use std::cell::RefCell;
#[allow(unused_imports)]
use std::rc::Rc;

pub struct Item {
    pub name: String,
    pub value: i32,
}

// provide a ctor for the structure defined
impl Item {
    #[allow(dead_code)]
    fn new(name: String, value: i32) -> Rc<RefCell<Item>> {
        Rc::new(RefCell::new(Item {
            name: name,
            value: value,
        }))
    }
}

#[test]
fn demo_internal_mutability() {
    let item = Item::new("e1m1".to_string(), 0);
    item.borrow_mut().value = 12123;
    assert_eq!(item.borrow().value, 12123);
}
