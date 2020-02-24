use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// algorithms with rust L703
// RefCell.RefCell maintains single ownership of a value but
// allows mutable borrowing checked at runtime
// instead of compiler errors, violating the rules of borrowing
// will lead to a runtime panic!() crashing the program

// Rc<RefCell<T>> combination is equivalent to C++ std::shared_ptr<T>
// void demo_modify_data_via_shared_ptr() {
//     int* data = new int(1);
//     auto ptr = shared_ptr<int>(data);
//  -- ptr = Rc::new(RefCell::new(..data..))
//     (*ptr) += 10234;
//  -- ptr.borrow_mut() += 10234
//     cout << *data << endl;
// }
fn _demo_shared_ptr() {
    let v = 23;
    let ptr = Rc::new(RefCell::new(v)); // owns 23!
    *(ptr.borrow_mut()) += 10232;
    println!("v: {}", ptr.borrow());
}
// std::cell
// Shareable mutable containers.

// std::cell::RefCell
// A mutable memory location with dynamically checked borrow rules

// this will cause panic!
// this violates the rules of borrowing (mut, immut ref co exists)
// - Having several immutable references (&T) to the object
//   (also known as aliasing).
// - Having one mutable reference (&mut T) to the object
//   (also known as mutability).

// ecause RefCell<T> borrows are dynamic it is possible to attempt
// to borrow a value that is already mutably borrowed; when this
// happens it results in thread panic
fn _append_str(mut sut: std::cell::RefMut<String>, token: String) {
    sut.push_str(&token);
}

// Values of the Cell<T> and RefCell<T> types may be mutated
// through shared references (i.e. the common &T type), whereas
// most Rust types can only be mutated through unique (&mut T)
// references. We say that Cell<T> and RefCell<T> provide
// 'interior mutability', in contrast with typical Rust types
// that exhibit 'inherited mutability'.
fn _demo_refcell() {
    let s = String::from("thereisacow");
    let mut t = s.clone();

    let t_ref = &mut t; // can only be mutated through unique (&mut T)
    let s_ref = RefCell::new(s);
    let s_ref_ = &s_ref; // may be mutated through shared references

    // borrow mutably, even the original s is immutable - it
    // does not matter because s_ref now owns the content of
    // the string!
    s_ref_.borrow_mut().push_str(" 1337");

    t_ref.push_str(" )x3234");

    println!("{}", s_ref.borrow());
    println!("{}", t_ref);
}

// Many shared smart pointer types, including Rc<T> and Arc<T>,
// provide containers that can be cloned and shared between multiple
// parties. Because the contained values may be multiply-aliased,
// they can only be borrowed with &, not &mut. Without cells it
// would be impossible to mutate data inside of these smart pointers
// at all.
// It's very common then to put a RefCell<T> inside shared pointer
// types to reintroduce mutability:
fn _demo_hashmap_shared_pointer() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    shared_map.borrow_mut().insert("africa", 92388);
    shared_map.borrow_mut().insert("kyoto", 11837);
    shared_map.borrow_mut().insert("piccadilly", 11826);
    shared_map.borrow_mut().insert("marbles", 38);
    println!("{:?}", shared_map.borrow());
}

fn _demo_interior_mutability() {
    let s = Rc::new(RefCell::new(String::from("there is")));

    // immutable borrow (immut ref)
    assert_eq!(*s.borrow(), "there is");
    // L719
    // this mutable reference only lives as long as the function
    // call takes (push_str()),
    // thereby ruling out creating too-large scope and violating
    // the borrowing rules
    s.borrow_mut().push_str(" a cow !");

    // immutable borrow (returns an immutable reference)
    assert_eq!(*s.borrow(), "there is a cow !");
}

fn _demo_into_inner() {
    let cell = RefCell::new(5);
    let v1 = cell.into_inner();
    // 32 |     let v1 = cell.into_inner();
    //    |              ---- value moved here
    // 33 |     let v2 = cell.into_inner();
    //    |              ^^^^ value used here after move
    // let v2 = cell.into_inner();
    assert_eq!(v1, 5);
}

#[test]
fn demo_all() {
    _demo_shared_ptr();
    _demo_hashmap_shared_pointer();
    _demo_refcell();
    _demo_interior_mutability();
    _demo_into_inner();
}
