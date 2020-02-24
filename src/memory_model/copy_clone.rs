#[derive(Clone)]
struct StringHolder {
    v: String,
}

fn _demo_clone() {
    // algorithms with rust L1276
    // when a variable is assigned to another variable, the
    // compiler will typically copy the value implicitly
    // copy is an implicit, bitwise copy of the value of a
    // variable. If that variable is a pointer, the memory
    // repsonsibility becomes ambiguous (who takes care of
    // freeing??) and compilation will fail
    // this is where clone() comes in,
    // the trait requires an explicit implementation of clone()
    // function to provide an appropriate copy of the type
    let sh1 = StringHolder {
        v: String::from("acow"),
    };
    println!("orig  {:p}", &(sh1.v));

    // clone is always a deep copy of a type
    let sh2 = sh1.clone();
    println!("clone {:p}", &(sh2.v));
    let sh3 = &sh1;
    println!("ref   {:p}", &(sh3.v));

    // it is recommended to derive or implement copy wherever
    // possible, but be mindful of breaking changes
    // adding the trait is a non-intrusive action, whereas
    // removing the trait will potentially break other code
}

#[test]
fn demo_all() {
    _demo_clone();
}
