pub fn iterator_sum() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum: i32 = nums.iter().sum();
    println!("Sum: {}", sum);
}

#[test]
fn demo_iterator_sum() {
    iterator_sum();
}
