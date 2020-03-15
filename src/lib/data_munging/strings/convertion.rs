fn _demo_string_to_u8(sut: String) {
    let number: u8 = match sut.parse() {
        Ok(n) => n,
        Err(_) => 0,
    };
    println!("{} parsed as {}", sut, number);
}

fn _demo_i64_from_hexstring(sut: String) {
    // source
    // https://stackoverflow.com/questions/32381414/converting-a-hexadecimal-string-to-a-decimal-integer
    // how to strip 0x prefix
    let without_prefix = sut.trim_start_matches("0x");
    let number: i64 = match i64::from_str_radix(&without_prefix, 16) {
        Ok(n) => n,
        Err(_) => 0,
    };
    println!("0x{:X}", number);
}

#[test]
fn demo_all() {
    _demo_string_to_u8("1".to_string());
    _demo_string_to_u8("asd".to_string());
    _demo_string_to_u8("1F".to_string());

    _demo_i64_from_hexstring("badbeef".to_string());
    _demo_i64_from_hexstring("0xbabe".to_string());
}
