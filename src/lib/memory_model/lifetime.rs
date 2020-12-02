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
