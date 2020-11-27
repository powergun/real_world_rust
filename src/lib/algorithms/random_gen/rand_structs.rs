// source
// rust std lib cookbook P/32
// see:
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html

// the trait described in the book is obsolete!

// UPDATE:
// here is the working example of randomly generating a custom
// type T:
// https://www.reddit.com/r/rust/comments/g4789z/i_feel_stupid_but_where_does_the_rand_trait_come/
// the gist is to implement the Distribution trait, instantiated
// by <T>, for type Standard
// once this is done, I can follow:
// https://rust-random.github.io/book/guide-start.html
// and call `let x: T = some_rng.gen()`
// some_rng is typically thread_rng()

extern crate rand;

use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::Add;

#[derive(Debug)]
struct P {
    x: i32,
    y: i32,
}

impl Add for P {
    type Output = P;

    fn add(self, other: P) -> Self::Output {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Distribution<P> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> P {
        P {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

#[test]
fn demo_random_points() {
    fn gen_point() -> P {
        let mut tr = rand::thread_rng();
        tr.gen()
    }
    // P { x: 1007184095, y: -1423601094 }
    // P { x: 1129930495, y: 875041174 }
    // P { x: -625306968, y: 630884006 }
    // P { x: 1595841640, y: -1419076123 }
    println!("{:?}", gen_point());
    println!("{:?}", gen_point());
    println!("{:?}", gen_point());
    println!("{:?}", gen_point());
}
