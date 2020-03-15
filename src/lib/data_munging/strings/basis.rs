fn _iter_char() {
    // creation
    // each char is a unicode scalar value
    // https://doc.rust-lang.org/std/primitive.char.html

    // let s = "aøπ"; can work too (compiled and executed),
    // but they are not the same type; <-- this is a &str
    // (pronaunced anstra)
    // passing &str avoid copying the characters
    // "aøπ" is stored statically in the binary (recall how
    // C handles global strings),
    // for example:
    // rename this string to thereisacow then:
    // strings basis.rs.bin | grep thereisacow
    let s = String::from("aøπ");

    // String has push_str methods, &str does not

    println!("length: {}", s.len());

    // one char can be more than a byte
    for c in s.chars() {
        println!("as char: {}", c); // 1 ascii char, 2 unicode chars
    }
    for bt in s.bytes() {
        println!("as byte: {}", bt); // for "aøπ" the total bytes are 1 + 4
    }
    // use enumeration
    let mut _sum = 0;
    for (i, _c) in s.chars().enumerate() {
        _sum += i; // 0 1 2
    }
    println!("sum: {}", _sum);

    // use character indices (in respect to the byte layout)
    // note the second and third character has 2 bytes hence
    // their indices 1 3
    for (i, _c) in s.char_indices() {
        println!("{}, {}", i, _c);
    }
}

// taking &str to enforce immutability and avoid copying
fn _count_e(s: &str) -> u32 {
    let mut sum = 0;
    for ch in s.chars() {
        if ch == 'e' {
            sum += 1;
        }
    }
    return sum;
}

fn _pointer_to_string() {
    println!("count: {}", _count_e("thereisacow"));
    let words = String::from("thereisdoublecow");
    // create a const pointer to words
    println!("count: {}", _count_e(&words));
}

#[test]
fn demo_all() {
    _iter_char();
    _pointer_to_string();
}
