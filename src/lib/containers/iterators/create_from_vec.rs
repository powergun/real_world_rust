// iterator trait
// only requires the implementation of .next() method

// iterator provided by standard library and built-in types
// .iter() -> immutable references
// .iter_mut() -> mutable references

// consuming iterators: sum, max...

// the reduce part of map-reduce: fold

// chaining
#[test]
fn demo_iterator_sqr_sum() {
    let xs = vec![3, 1, 4, 1, 5, 9];
    let sqr_sum: i32 = xs.iter().map(|x| x * x).sum();
    assert_eq!(133, sqr_sum);
}

// explicit evaluation
// collect() method turns an iterator into a Vec or some other
// collections that implement the FromIter trait
#[test]
fn demo_iterator_map_collect() {
    // map() is not evaluated unless something consumes its
    // result; or calling .collect() will trigger evaluation
    let xs = vec![3, 1, 4, 1, 5, 9];
    let plus_one_iter = xs.iter().map(|elem| elem + 1);

    let ys: Vec<_> = plus_one_iter.collect();
    assert_eq!(vec![4, 2, 5, 2, 6, 10], ys);
}
