// rust std cookbook P/17
// ctor is just a convention rather than a rule
// rust's std lib uses pattern very often

#[allow(dead_code)]
struct Item {
    name: String,
    value: u32,
}

impl Item {
    // P/19
    // if a struct provides a method new() that returns Self,
    // the user of the struct will not depend on the members
    // of the struct, as they are considered an internal
    // hidden state
    #[allow(dead_code)]
    fn new(name: String, value: u32) -> Self {
        // rust supports "field init shorthand syntax"
        // see: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
        Item{name, value}
    }

    // setter method: requires mutable ref, & mut self
    #[allow(dead_code)]
    fn set_value(& mut self, value: u32) {
        self.value = value;
    }

    // getter method: requires immutable ref, &self
    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.value == 0
    }
}

#[test]
fn demo_ctor_pattern() {
    let mut i = Item::new("map".to_string(), 23);
    assert_eq!(23, i.value);

    i.set_value(123);
    assert_eq!(123, i.value);

    assert_eq!(false, i.empty());
}
