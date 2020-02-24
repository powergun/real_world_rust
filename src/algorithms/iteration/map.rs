// recall map {} in perl and ruby
pub fn iterator_map() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let operand = 1;
    nums.iter()
        .map(|x| x + operand)
        .for_each(|x| print!("{}, ", x));
    // map() is a closure; it can use the variables in the
    // calling scope; same applies to filter()
    println!("");

    // the original vector is unmodified
    nums.iter().for_each(|elem| print!("{}, ", elem));
    println!("");
}

#[test]
fn demo_iterator_map() {
    iterator_map();
}
