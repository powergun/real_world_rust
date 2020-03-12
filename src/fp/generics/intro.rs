// FP in Rust L377

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub fn foo<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}

#[test]
pub fn test_create_points() {
    let p1 = Point { x: -1, y: -2 };
    assert_eq!(1, foo(p1.x));
    let p2 = Point { x: 0x10, y: 0x12 };
    assert_eq!(256, foo(p2.x));

    // beware of precision issue: 0.4 != 0.40000000000001
    let p3 = Point { x: 0.1, y: 10.0 };
    assert_eq!(100.0, foo(p3.y));
}
