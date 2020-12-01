#[allow(unused_imports)]
use rand::{Rng, thread_rng};
#[allow(unused_imports)]
use rand::distributions::Alphanumeric;

#[test]
fn demo_gen_random_strings() {
    fn gen(size: usize) -> String {
        // sample_iter() takes arg by value
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(size)
            .collect()
    }
    let x1 = gen(8);
    let x2 = gen(16);
    // println!("{} {}", x1, x2);
    assert_eq!(x1.len(), 8);
    assert_eq!(x2.len(), 16);
} 