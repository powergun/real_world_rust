// Prim types are cheap to copy therefore they are always copied in an assignment.
// Recall that Modern Effective C++ also recommends this approach

// Container/struct types may have pointer to heap memory. If they are copied (shallowly), it will
// cause all kinds of problems such as double-freeing.
// Therefore container/struct types are always moved in an assignment.

// How does rust enforce this:
// Prim types (and some container/struct types) implement the Copy trait, therefore it copies all
// the data it owns to another value; Struct (if not deriving or implementing the Clone trait)
// does not have the deeply-copy capability.

// I can not derive from the Copy trait on my custom struct either, because String does not impl
// the Copy trait. Deriving the Copy trait requires the struct to also derive/impl the Clone trait.

#[allow(dead_code)]
struct Item {
    s: String, // a pointer basically
}

#[allow(dead_code)]
#[derive(Clone)]
struct ItemCloneable {
    s: String, // a pointer basically

    // Note, in order to derive from Clone, all the members must also implement the Clone trait
    // won't compile:
    // the trait `std::clone::Clone` is not implemented for `memory_model::ownership_and_copying::Item`
    // i: Item,
}

#[test]
fn demo_to_copy_or_not_to_copy() {
    let i1 = 1;
    let i2 = i1; // copy (deeply)
    assert_eq!(i1, i2);

    let _item1 = Item { s: "e1m1".to_string() };
    let _item2 = _item1; // move; _item1 is invalid now
    // won't compile: value borrowed here after move
    // assert_eq!(item1.s, "e1m1");

    {
        let item1 = ItemCloneable { s: "e1m1".to_string() };
        let item2 = item1.clone(); // copy (deeply); the data pointed to is duplicated
        assert_eq!(item1.s, item2.s);
    }
}