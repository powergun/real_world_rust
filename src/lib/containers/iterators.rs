// iterator trait
// only requires the implementation of .next() method

// iterator provided by standard library and built-in types
// .iter() -> immutable references
// .iter_mut() -> mutable references

// consuming iterators: sum, max...

// the reduce part of map-reduce: fold

// chaining
pub fn iterator_sqr_sum() {
    let nums = vec![3, 1, 4, 1, 5, 9];
    let sqr_sum: i32 = nums.iter().map(|x| x * x).sum();
    println!("Squared-sum: {}", sqr_sum);
}

// explicit evaluation
// collect() method turns an iterator into a Vec or some other
// collections that implement the FromIter trait
pub fn iterator_map_collect() {
    // map() is not evaluated unless something consumes its
    // result; or calling .collect() will trigger evaluation
    let nums = vec![3, 1, 4, 1, 5, 9];
    let plus_one_iter = nums.iter().map(|elem| elem + 1);
    println!("Iter-map: {:?}", plus_one_iter);
    let modified: Vec<i32> = plus_one_iter.collect();
    println!("Modified: {:?}", modified);
}

pub fn iterator_infinite() {
    let nums: Vec<i32> = (1..) // 1 to infinity
        .map(|elem| elem + 1) // transform
        .filter(|elem| elem % 5 == 0) // transform
        .take(7) // take the first seven values computed
        .collect(); // trigger evaluation
    println!("Seven: {:?}", nums);
}

#[test]
fn demo_all() {
    iterator_sqr_sum();
    iterator_map_collect();
    iterator_infinite();
}
