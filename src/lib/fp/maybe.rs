#[test]
fn demo_take_from_optional() {
    // take() is used to preserve the structure while updating the inner value, particularly for
    // a recursive type like LinkedList
    let mut x: Option<_> = Some(1);
    let _o = x.take(); // _o is Option<i32> - it is the same value wrapped in Maybe
    assert!(x.is_none());
}
