// taking &str to enforce immutability and avoid copying
#[allow(dead_code)]
fn count_e_character(s: &str) -> u32 {
    let mut sum = 0;
    for ch in s.chars() {
        if ch == 'e' {
            sum += 1;
        }
    }
    return sum;
}

#[test]
fn demo_function_param_string_slice() {
    assert_eq!(2, count_e_character("there is a cow"));

    // if I have a string variable instead of &str, I can use
    // & to create a const pointer to it
    let words = String::from("there is a cow eee");
    assert_eq!(5, count_e_character(&words));
}
