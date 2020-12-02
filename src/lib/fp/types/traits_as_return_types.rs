trait Animal {
    fn make_noise(&self) -> i32;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_noise(&self) -> i32 {
        1
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> i32 {
        2
    }
}

// can not return bare trait - the compiler does not know the
// precise size;
// must use a Boxed type with dyn keyword
#[allow(dead_code)]
fn get_animal(x: i32) -> Box<dyn Animal> {
    match x {
        10..=20 => Box::new(Dog {}),
        _ => Box::new(Cat {}),
    }
}

#[test]
fn demo_traits_as_return_types() {
    assert_eq!(get_animal(15).make_noise(), 1);
    assert_eq!(get_animal(12321).make_noise(), 2);
}
