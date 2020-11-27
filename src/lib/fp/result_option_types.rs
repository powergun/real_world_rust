#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::env::args; // args() function

#[test]
fn use_optional_return_value_from_hashmap() {
    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("map".to_string(), "e1m1".to_string());

    // case-match optional
    let r1 = match hm.get("name") {
        Some(x) => x,
        _ => "",
    };
    assert_eq!(r1, "");

    // unsafe; will panic
    let r2 = hm.get("map").unwrap();
    assert_eq!(r2, "e1m1");

    // safe; fallback to default
    let defa = "e2m2".to_string();
    let r3 = hm.get("name").unwrap_or(&defa);
    assert_eq!(r3, "e2m2");

    // unsafe with specific panic message
    hm.get("map").expect("map key does not exist!!");
}

#[test]
fn use_result_type_from_parse() {
    // parse with turbo-fish
    let x: i32 = match "3".parse::<i32>() {
        Ok(x) => x,
        
        // Err wraps an err description
        Err(_) => -1,
    };
    assert_eq!(x, 3);

    // use safe unwrap_or
    let y = "3xxx".parse::<i32>().unwrap_or(-1);
    assert_eq!(y, -1);

    // unsafe expect() applies here too
}

#[test]
fn return_result_type() {
    fn f() -> Result<i32, i32> {
        let x = args()
                .enumerate()
                .any(|(idx, _argument)| {idx > 10});
        if x {
            Ok(1)
        } else {
            Err(-1)
        }
    };
    assert_eq!(f(), Err(-1));
}
