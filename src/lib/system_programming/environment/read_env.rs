// see also bin/cat.rs for another simple example

// also explained in:
// rust std lib cookbook P/38

// see also:
// https://crates.io/crates/dotenv

#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::env::{set_var, var};

#[test]
fn demo_iterate_over_env_vars() {
    let vars = env::vars();
    for (env_var_name, _env_var_value) in vars {
        assert_ne!("asd", env_var_name);
    }
}

#[test]
fn demo_safe_read_one_env_var() {
    let f = |name: String| -> String {
        match env::var(name) {
            Ok(val) => val,
            Err(_) => String::new(),
        }
    };
    assert_eq!("", f("asdasd".to_string()));
    assert_eq!("/bin/bash", f("SHELL".to_string()));

    // P/39
    // how to use the Result type: use unwrap_or_default()
    // function to streamline the getter subroutine
    assert_eq!("", env::var("asdasd").unwrap_or_default());
    assert_eq!("/bin/bash", env::var("SHELL").unwrap_or_default());

    // use unwrap_or
    assert_eq!("XX", env::var("asdasd").unwrap_or("XX".to_string()));
}

#[test]
fn set_env_var() {
    set_var("ROAD", "e1m1");
    let v = var("ROAD").unwrap_or("....".to_string());
    assert_eq!(v.len(), 4);
}
