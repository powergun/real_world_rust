// behave similarly to ruby/crystal's reduce()
pub fn iterator_fold() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sum: i32 = nums.iter().fold(
        0,
        |sum, val| sum + val, // init accu elem
    );
    println!("Reduce-sum: {}", sum);
}

#[test]
fn demo_iterator_fold() {
    iterator_fold();
}
