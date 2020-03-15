// alias
pub type Name = String;

// alias to tuple (python's tuple)
pub type Item = (i32, String);

#[test]
pub fn test_aliases() {
    let _n = Name::from("asd");
    let _i: Item = (1, "asd".to_string());
}
