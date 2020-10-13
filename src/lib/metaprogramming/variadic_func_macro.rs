// source:
// rust std lib cookbook P/43

#[allow(unused_macros)]
macro_rules!  multiply_many{
    // tail case
    ( $last:expr ) => {
        $last
    };

    // recursive call
    ( $head:expr, $($tail:expr), +) => {
        $head * multiply_many!($($tail), +)
    };
}

#[test]
fn demo_mul_many() {
    let o = multiply_many!(1, 2, 1, 2, 1, 2);
    assert_eq!(8, o);

    // tail case:
    // multiply_many(2) => 2
    // haskell: mut [x] = x

    // recursive case:
    // $head, $tail, + => $head * multiply_many($tail, +)
    // haskell: mut (hd::tl) = hd * mut(tl)
}

// $f is an identifier not an expression; therefore I can pass
// in a function (even if the function is a lambda)
// , separate the patterns
// note that I still have to expand the tail bit like the above
// demo
#[allow(unused_macros)]
macro_rules! apply_void {
    ($f:ident, $last:expr) => {
        $f($last)
    };

    ($f:ident, $head:expr, $($tail:expr), +) => {
        $f($head);
        apply_void!($f, $($tail), +)
    };
}

#[test]
fn demo_apply_void() {
    let f = |x| {
        println!("//////////// {:?}", x);
    };
    apply_void!(f, 1, 2, 3);
}
