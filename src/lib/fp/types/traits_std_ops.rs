
// std::ops::AddAssign

#[test]
fn demo_std_ops_traits() {
    fn f<T: std::ops::AddAssign>(mut x: T, delta: T) -> T {
        x += delta;
        x
    }
    assert_eq!(f(1, 2), 3);
}