// put the use statement here because HashMap is only used
// inside this module scope
use std::collections::HashMap;

pub fn fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    match n {
        0 | 1 => 1,
        n => {
            if map.contains_key(&n) {
                *map.get(&n).unwrap()
            } else {
                let val = fib(n - 1, map) + fib(n - 2, map);
                map.insert(n, val);
                val
            }
        }
    }
}

#[test]
fn demo_fib_by_hashmap() {
    let mut hm = HashMap::new();
    (40..42).for_each(|x| {
        // with the hashmap optimization this will hang for a
        // long time
        for _ in 1..1000 {
            assert!(fib(x, &mut hm) > 0);
        }
    });
}
