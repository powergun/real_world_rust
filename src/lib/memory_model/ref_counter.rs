use std::rc::Rc;

fn _demo_ref_counter() {
    // can R-access in many places
    let _rc = Rc::new("data");
}

#[derive(Debug)]
struct Filename {
    name: Rc<String>,
    ext: Rc<String>,
}

// algorithms with rust L645
// one solution is to clone the object in every iteration, but
// that causes a lot of slow mallocs; for this rust std provides
// ref counting
// Rc encapsulates a variable of type T allocated on the heap
// and returns an immutable reference when created
// this ref can be cloned with low overhead (it's only as ref
// count that is incremented) but never transformed into a
// mutable ref
// regardless it acts just like owned data, passing through
// function calls and property lookups

// works great for single-threaded and immutable scenarios
// but will refuse to compile multithreaded code
fn _demo_multiple_ownership() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));
    for _ in 0..3 {
        println!(
            "{:?}",
            Filename {
                name: name.clone(),
                ext: ext.clone()
            }
        );
    }
}

#[test]
fn demo_all() {
    _demo_ref_counter();
    _demo_multiple_ownership();
}
