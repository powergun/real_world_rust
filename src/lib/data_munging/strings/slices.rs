// string slices are immutable

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

#[test]
fn demo_static_string_slice() {
    fn f() -> &'static str {
        "e1m1"
    }
    let ss: &'static str = f();
    assert!(ss.len() > 0);
}

#[test]
fn demo_string_slice_and_str() {
    let mut s: String = "e1m1".to_string();
    let sub1 = &s[1..3]; // sub1 is of &str; range [1..3] is exclusive [1, 3)
    assert_eq!(sub1, "1m");

    let _sub2 = &mut s[1..3];
}