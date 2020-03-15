use std::env;

fn _display_raw_arguments() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

fn _parse_arguments_from_scratch() {
    // see
    // https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html

    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let cleartext = &args[1];
            let key = &args[2];
            // parse the _number
            let _number: i32 = match key.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    return;
                }
            };
            println!("{} / {}", cleartext, key);
        }
        // all the other cases
        _ => {
            println!("usage: caesar <cleartext> <key>");
        }
    }
}

#[test]
fn demo_all() {
    _display_raw_arguments();
    _parse_arguments_from_scratch();
}
