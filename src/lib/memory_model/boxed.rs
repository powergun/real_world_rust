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
