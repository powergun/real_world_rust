// see also bin/cat.rs for another simple example

use std::env;

fn _demo_read_env_var() {
    let vars = env::vars();
    for (k, v) in vars {
        println!("{}={}", k, v);
    }
}

#[test]
fn demo_all() {
    _demo_read_env_var();
}
