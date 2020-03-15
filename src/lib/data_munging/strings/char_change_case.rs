// note to_ascii_uppercase() does not need 'use std::ascii::AsciiExt'
// this module is deprecated

fn _demo_char_change_case(s: &String) {
    let mut result = String::new();
    s.chars().for_each(|c| result.push(c.to_ascii_uppercase()));
    println!("{}", result);
}

#[test]
fn demo_all() {
    _demo_char_change_case(&"there is acow".to_string());
}
