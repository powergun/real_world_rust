use std::io;

fn _demo_take_input() {
    let mut text = String::new();
    while io::stdin().read_line(&mut text).unwrap() > 1 {
        // if the input is to be parsed as numeric value, it
        // must be trimmed
        text = text.trim().to_string();
        println!("input: {}", text);
        text.clear();
    }
    println!("+ done");
}

#[test]
fn demo_all() {
    _demo_take_input();
}
