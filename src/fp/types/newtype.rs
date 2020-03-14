// haskell-like newtype
// see also:
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
// https://doc.rust-lang.org/1.0.0/style/features/types/newtype.html
pub struct Label(String);

// python's named tuple
pub struct Record(i32, String);

#[test]
pub fn test_newtype() {
    let _l = Label("asd".to_string());
    let _r = Record(1, "asd".to_string());
}
