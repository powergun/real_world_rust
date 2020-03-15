pub fn demo_take() {
    // source
    // https://doc.rust-lang.org/std/iter/trait.FromIterator.html
    use std::iter::FromIterator;

    let five_fives = std::iter::repeat(5).take(5);
    let v = Vec::from_iter(five_fives);
    assert_eq!(v, vec![5, 5, 5, 5, 5]);

    let iterator = (1..10).into_iter();
    let mut taken = iterator.take(2);
    while let Some(n) = taken.next() {
        println!("taken: {}", n);
    }
}

// recall crystal/ruby: with_index
pub fn demo_enumerate() {
    let mut it = (1..10).take(3).enumerate();
    while let Some(n) = it.next() {
        println!("with index: {} - {}", n.0, n.1);
    }
}

#[test]
fn demo_all() {
    demo_take();
    demo_enumerate();
}
