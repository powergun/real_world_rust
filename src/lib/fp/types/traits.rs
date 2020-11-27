#[allow(unused_imports)]
use std::ops::Add;

#[allow(dead_code)]
struct P {
    x: i32,
    y: i32,
}

impl Add for P {
    // see: https://doc.rust-lang.org/beta/std/ops/trait.Add.html
    type Output = Self;

    fn add(self, other: P) -> Self {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_add_trait_for_point() {
    let a = P { x: 3, y: 5 };
    let b = P { x: 10, y: 20 };
    let c = a + b;

    assert_eq!(c.x, 13);
}
