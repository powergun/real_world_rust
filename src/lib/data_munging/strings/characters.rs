#[test]
fn demo_string_from() {
    // creation
    // each char is a unicode scalar value
    // https://doc.rust-lang.org/std/primitive.char.html

    // let s = "aøπ"; can work too (compiled and executed),
    // but they are not the same type; <-- this is a &str
    // (pronounced anstra)
    // passing &str avoid copying the characters
    // "aøπ" is stored statically in the binary (recall how
    // C handles global strings),
    // for example:
    // rename this string to thereisacow then:
    // strings basis.rs.bin | grep thereisacow
    let s = String::from("aøπ");

    // String has push_str methods, &str does not

    // len() returns the byte length (not number of chars)
    assert_eq!(5, s.len());

    // one char can be more than a byte
    // here it has: 1 ascii char, 2 unicode chars
    assert_eq!(3, s.chars().count());

    // for "aøπ" the total bytes are 1 + 4
    assert_eq!(5, s.bytes().count());

    // use enumeration()
    {
        let mut iter = s.chars().enumerate();
        assert_eq!(0, iter.next().unwrap().0);
        assert_eq!(1, iter.next().unwrap().0);
        assert_eq!(2, iter.next().unwrap().0);
    }

    // use character_indices() (in respect to the byte layout)
    // note the second and third character has 2 bytes hence
    // their indices 1 3
    {
        let mut iter = s.char_indices();
        assert_eq!(0, iter.next().unwrap().0);
        assert_eq!(1, iter.next().unwrap().0);
        assert_eq!(3, iter.next().unwrap().0);
    }
}
