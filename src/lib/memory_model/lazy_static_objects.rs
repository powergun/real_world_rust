// rust std lib cookbook P/141
// big objects, especially constant ones, should be reused
// instead of rebuilt

// https://docs.rs/lazy_static/1.4.0/lazy_static/

// why lazy static is useful:
// P/145
// in rust, static var has to be built in a constant way, that
// is, a way that is known at compile time.
// it is impossible to build a hashmap during compile time
// another catch with static var is that, because they have
// a global life time, the borrow checker can not make sure
// that their access is thread-safe;
// as a consequence, any access on a static mut var will always
// be unsafe.

// you can view a `normal static rust` as a `static constexpr c++`
// and a `lazy_static rust` as a `static c++ local`

#[test]
fn demo_() {}
