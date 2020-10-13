// source
// rust std lib cookbook P/33

#[test]
fn demo_replace_by_capture_groups() {
    use regex::Regex;
    let text = r"  data SomeTemplate =   SomeTemplate   with ";
    //           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //           the range of characters being replaced
    let regex = Regex::new(r"^\s*data\s+(\w+)\s*=\s*(\w+).+").unwrap();
    // for g in regex.captures_iter(text) {
    //     println!("{:?}", g);
    // }
    let text_new = regex.replace_all(text, "class $1 = $2 :: ");
    //                                      ^^^^^^^^^^^^^^^^^
    //                                      the new string that
    //                  replaces the original range of characters
    assert_eq!("class SomeTemplate = SomeTemplate :: ", text_new);
}

// source: P/33
#[test]
fn demo_replace_by_named_capture_groups() {
    use regex::Regex;
    let text = r"  data SomeTemplate =   SomeTemplate   with ";
    let regex = Regex::new(r"^\s*data\s+(?P<type_name>\w+)\s*=\s*(?P<data_ctor>\w+).+").unwrap();
    let text_new = regex.replace_all(text, "class $type_name = $data_ctor :: ");
    assert_eq!("class SomeTemplate = SomeTemplate :: ", text_new);
}
