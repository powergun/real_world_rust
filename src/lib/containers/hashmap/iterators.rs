#[allow(unused_imports)]
use std::collections::HashMap;

#[test]
fn demo_iter_key_values() {
    let mut tb = HashMap::new();
    vec!["map", "creature", "prop"].iter().for_each(|x| {
        tb.insert(x.to_string(), x.len());
    });

    // read only iterating
    for (k, &v) in &tb {
        assert_eq!(k.len(), v);
    }

    // mutable iterating
    for v in tb.values_mut() {
        *v *= 100;
    }
    // for (_, v) in &mut tb {
    //     *v *= 100;
    // }
    assert_eq!(300, *tb.get("map").unwrap());

    // consume iterating
    for _ in tb {}
    // won't compile
    // tb has been consumed
    // assert_eq!(3, tb.len());
}
