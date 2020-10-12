// source:
// rust std cookbook P/20

// also a good read:
// https://jwilm.io/blog/from-str-to-cow/

use std::borrow::Cow;

#[allow(dead_code)]
struct Size<'a> {
    name: Cow<'a, str>,
    length: usize,
}

impl<'a> Size<'a> {
    #[allow(dead_code)]
    fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>, // S must have Into trait;
    {
        let name: Cow<'a, str> = name.into();
        Size {
            length: name.len(),
            name,
        }
    }
}

#[test]
fn demo_cow() {
    let s1 = Size::new("asd");
    assert_eq!(3, s1.length);
}
