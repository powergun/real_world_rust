// rust std lib cookbook P/34
//

#[test]
fn demo_iter_capture_groups() {
    // P/34
    // the [0] index is always the entire capture;
    // the [1] index is then the string from our first capture
    use regex::Regex;
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }
}
