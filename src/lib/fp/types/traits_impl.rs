#[allow(unused_imports)]
use std::ops::Add;

#[allow(dead_code)]
struct P {
    x: i32,
    y: i32,
}

// this is a form of operator-overloading
impl Add for P {
    // see: https://doc.rust-lang.org/beta/std/ops/trait.Add.html
    type Output = Self;

    fn add(self, other: P) -> Self::Output {
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

// I can also impl a new trait for a type not of my own
// the new trait MUST BE defined in the current crate
// for example, I CAN NOT impl the Add crate for Vec<T>

trait Collapse<T: Add> {
    fn collapse(&self) -> T;
}

// static dispatch
impl Collapse<i32> for Vec<i32> {
    fn collapse(&self) -> i32 {
        self.iter().fold(0, |acc, x| acc + x)
    }
}

// dynamic dispatch
#[allow(dead_code)]
fn col<T>(_xs: &dyn Collapse<T>) {}

#[test]
fn demo_use_new_trait_for_vector() {
    let xs: Vec<i32> = vec![1, 2, 3, 4];
    //       ^^^^^ can not be usize or i64

    // xs is statically converted to Collapse<i32> at compile
    // time
    assert_eq!(10, xs.collapse());

    // use dynamical dispatch at runtime
    col(&xs);
}
