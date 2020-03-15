pub fn iterator_max() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    // max() returns an option instead of a real value
    let max: &i32 = nums.iter().max().unwrap();
    println!("Max: {}", max);

    let empty: Vec<i32> = [].to_vec();
    // if the container is empty, max() returns None
    println!("{:?}", empty.iter().max());
}

#[test]
fn demo_iterator_max() {
    iterator_max();
}
