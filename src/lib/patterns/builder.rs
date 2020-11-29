// the builder pattern
// take a mut ref, return a mut ref

#[allow(dead_code)]
struct Payload {
    a: i32,
    b: i32,
    c: i32,
}

#[allow(dead_code)]
impl Payload {
    fn new() -> Self {
        Payload { a: 0, b: 0, c: 0 }
    }

    fn build_a(&mut self) -> &mut Self {
        self.a = 123;
        self
    }
    fn build_b(&mut self) -> &mut Self {
        self.b = 123;
        self
    }
    fn build_c(&mut self) -> &mut Self {
        self.c = 123;
        self
    }

    fn to_string(&self) -> String {
        format!("{} {} {}", self.a, self.b, self.c)
    }
}

#[test]
fn demo_builder_pattern() {
    let s = Payload::new().build_a().build_b().build_c().to_string();
    assert_eq!(s, "123 123 123".to_string());
}
