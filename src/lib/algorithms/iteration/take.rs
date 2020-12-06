#[test]
fn demo_take() {
    // source
    // https://doc.rust-lang.org/std/iter/trait.FromIterator.html
    use std::iter::FromIterator;

    let five_fives = std::iter::repeat(5).take(5);
    let v = Vec::from_iter(five_fives);
    assert_eq!(v, vec![5, 5, 5, 5, 5]);

    let iterator = (1..10).into_iter();
    let taken = iterator.take(2);

    // avoid using this procedural style!
    // and to make things worse, it requires the iterator to be mutable!
    // while let Some(n) = taken.next() {
    //     println!("taken: {}", n);
    // }

    assert_eq!(taken.collect::<Vec<i32>>(), vec![1, 2]);
}

#[test]
fn demo_enumerate() {
    // recall crystal/ruby: with_index

    let it = (1..10).take(3).enumerate();

    // same, avoid this procedural style (and using mutable iter)
    // while let Some(n) = it.next() {
    //     println!("with index: {} - {}", n.0, n.1);
    // }

    assert_eq!(it.collect::<Vec<(usize, i32)>>(), vec![(0, 1), (1, 2), (2, 3)]);
}
