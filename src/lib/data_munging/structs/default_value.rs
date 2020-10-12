// source:
// rust std cookbook P/15

// derive the Default trait
// P/17
// nearly every type in Rust has a Default impl
// when you define your own struct that only contains elements
// that already have a Default (trait), you have the option to
// derive from Default as well
#[allow(dead_code)]
#[derive(Default)]
struct Item {
    label: Label,
    value: u32,
}

#[allow(dead_code)]
struct Label {
    value: String,
}

// implement the Default trait for Label type
impl Default for Label {
    fn default() -> Label {
        Label {
            value: "unknown".to_string(),
        }
    }
}

#[test]
fn demo_default_value() {
    let i: Item = Default::default();
    assert_eq!("unknown", i.label.value);
    assert_eq!(0, i.value);
}
