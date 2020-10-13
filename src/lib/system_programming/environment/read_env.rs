// see also bin/cat.rs for another simple example

// also explained in:
// rust std lib cookbook P/38

#[allow(unused_imports)]
use std::env;

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
}
