pub use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct Item {
    value: i32,
}

// This function borrows a slice
pub fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn implicit_copy_from_slice(slice: &[Item]) {
    let mut _i = slice[0]; // making a copy from the borrowed slice
    _i.value = 999; // changing the local copy
}

#[test]
fn demo_all() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    // the default value must be a const or literal (like in
    // C++); it must be known to the compiler
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the size of the array
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);

    let items: [Item; 2] = [Item { value: 1 }, Item { value: 2 }];
    implicit_copy_from_slice(&items[0..1]);
    println!("{:?}", items[0]) // unaffected by the modification above
}

#[test]
fn safe_modify_elements() {
    let mut _xs = [0; 5];

    // this causes a compile error!
    // _xs[7] = 1;
}

#[test]
fn array_iterator() {
    let xs = [0; 5];
    // can not collect to another array
    let ys = xs.iter().map(|x| x + 1).collect::<Vec<i32>>();
    assert_eq!(ys.len(), 5);
}
