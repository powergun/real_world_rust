#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Person;

#[allow(dead_code)]
struct Dog<'l> {
    owner: &'l Person,
}

#[test]
fn demo_life_time_a_has_b() {
    let p = Person {};
    let d = Dog { owner: &p };

    assert_eq!(p, *d.owner);

    let mut dm = Dog { owner: &p };
    {
        let p = Person {};
        dm.owner = &p;
    }
    // if p (Person) is used after this scoped modification
    // the code will not compile
    // assert_eq!(p, *dm.owner);
}

#[allow(dead_code)]
struct Map {
    name: String,
}

#[test]
fn demo_member_lifetime() {
    let mut m = Map { name: "e1m1".to_string() };
    let map_name = &m.name; // immut borrow as many times as I like

    // can not mut-borrow before the immut borrow is used
    // let mm = &mut m.name;
    assert_eq!(map_name, "e1m1");

    // immut-borrow is no longer used hence safe to mut-borrow
    let mm = &mut m.name;
    mm.push_str(" v1");
    assert_eq!(mm, "e1m1 v1"); // use the mut borrow
}
