// is a pointer to a block of memory
// (block of memory is commonly represented by array or vector)

// mutable slices allow use to change values

// allow passing around views into memory without copying values or
// passing raw pointers
//
// like vecs and strings, they consist of a pointer and a length, but
// they do not own the data they point to

// strs are like slices for strings
// also allow passing around a set of memory without copying

// slice and str are unsized for they don't own the memory
// they can only exist as references

pub fn demo_vector_slice() {
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8];
    let s1_nums = &nums[1..3]; // [1 -> 3) exclusive
    s1_nums.iter().for_each(|x| print!("{} ", x));
    println!("");
}

pub fn demo_string_slice() {
    let s = String::from("there is a cow");
    let s1_s = &s[2..4]; // exclusive
    println!("{}", s1_s);
}

#[test]
fn demo_all() {
    demo_vector_slice();
    demo_string_slice();
}

#[test]
fn demo_array_slice() {
    let mut xs = [0, 1, 2, 3, 4, 5];
    //                     0  1  2 indices of the slice
    let sxs = &xs[3..5];
    assert_eq!(sxs[0], 3);

    // slice size is determined at runtime

    {
        // out of bound
        // be careful, the code can be compiled but will crash at
        // runtime
        // let _sxs = &xs[7..10];
        // println!("{:?}", _sxs);
    }

    // update array element via slice

    {
        let sxs = &mut xs[1..3];
        // sxs is of type `&mut [i32]`
        sxs[0] = 999;
        // modification is reflected on the source array
        assert_eq!(xs, [0, 999, 2, 3, 4, 5]);
    }
}
