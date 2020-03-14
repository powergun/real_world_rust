#[test]
pub fn test_compact_expressions() {
    // FP in Rust L462
    let x: i32 = {
        fn f(x: i32) -> i32 {
            x * x
        }
        let y = f(5);
        y * 3
    };
    // FP in rust L470
    let z: i32 = if 1 > 2 { 324 } else { -1232 };
    assert_eq!(75, x);
    assert_eq!(-1232, z);
}
