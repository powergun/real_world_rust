// mentioned in
// rust std lib cookbook P/56

// string offers a basic parse() function

/////////////// from string ////////////////////////////////////
///
#[test]
fn demo_parse_to_u8() {
    let f = |sut: String| -> u8 {
        let number: u8 = match sut.parse() {
            Ok(n) => n,
            Err(_) => 0,
        };
        number
    };

    // success
    assert_eq!(1, f("1".to_string()));

    // fail
    assert_eq!(0, f("asd".to_string()));
    assert_eq!(0, f("1F".to_string()));
}

#[test]
fn demo_parse_to_i64() {
    let f = |sut: String| -> i64 {
        // source
        // https://stackoverflow.com/questions/32381414/converting-a-hexadecimal-string-to-a-decimal-integer
        // how to strip 0x prefix
        let without_prefix = sut.trim_start_matches("0x");
        let number: i64 = match i64::from_str_radix(&without_prefix, 16) {
            Ok(n) => n,
            Err(_) => 0,
        };
        number
    };

    // success
    assert_eq!(0xbadbeef, f("badbeef".to_string()));
    assert_eq!(0xbabe, f("0xbabe".to_string()));
}

//////////////// to string /////////////////////////////////////

#[test]
fn demo_num_to_string() {
    assert_eq!("1", 1.to_string());
}
