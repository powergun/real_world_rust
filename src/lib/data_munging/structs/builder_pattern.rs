// source
// rust std cookbook P/21

#[allow(dead_code)]
struct Item {
    name: String,
    value: u32,
}

#[allow(dead_code)]
#[derive(Default)]
struct ItemBuilder {
    name: String,
    value: u32,
}

impl ItemBuilder {
    #[allow(dead_code)]
    fn new() -> Self {
        ItemBuilder {
            name: "unnamed".to_string(),
            value: 10,
        }
    }

    // recall DA's spider project - this would be where ingress
    // validation happens... (pure functions only)
    #[allow(dead_code)]
    fn build(&self) -> Result<Item, String> {
        // "ingress validation"
        if self.name.len() == 0 {
            Err(format!("invalid name: {}", self.name))
        } else if self.value == 0 {
            Err(format!("invalid value: {}", self.value))
        } else {
            Ok(Item {
                name: self.name.clone(),
                value: self.value,
            })
        }
    }

    #[allow(dead_code)]
    fn with_name(mut self, v: &str) -> Self {
        self.name = v.to_string();
        self
    }

    #[allow(dead_code)]
    fn with_value(mut self, v: u32) -> Self {
        self.value = v;
        self
    }
}

#[test]
fn demo_builder_chain() {
    match ItemBuilder::new().build() {
        Ok(i) => assert_eq!(10, i.value),
        Err(_) => panic!("fail!"),
    };

    let result = ItemBuilder::new().with_name("e1m1").with_value(231).build();
    match result {
        Ok(i) => {
            assert_eq!("e1m1", i.name);
            assert_eq!(231, i.value);
        }
        Err(_) => panic!("fail"),
    }

    match ItemBuilder::new().with_value(0).build() {
        Ok(_) => panic!("shall not pass"),
        Err(error) => assert_eq!("invalid value: 0", error),
    }
}
