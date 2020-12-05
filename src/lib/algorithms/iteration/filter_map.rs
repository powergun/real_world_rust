
#[test]
fn demo_vector_map_filter() {
    // read:
    // https://doc.rust-lang.org/beta/std/iter/trait.Iterator.html#method.filter_map
    /*
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>ⓘ where
    F: FnMut(Self::Item) -> Option<B>,
    */
    // basically f is a function that returns an Option type;
    // recall Scala uses collect() and partial function to achieve the same map-and-filter operation
    let xs = vec![1, 2, 3, 4];
    let ys: Vec<_> = xs.iter()
        .filter_map(|x| {
            if *x > 3 {
                Some(3)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(ys, vec![3]);
}