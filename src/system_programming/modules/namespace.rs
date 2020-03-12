//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

mod doom {
    // default access level is private
    pub fn map(episode: i32, level: i32) -> String {
        format!("e{}m{}", episode, level)
    }
    // same applies to nested module
    pub mod creature {
        pub fn creature_name() -> String {
            "imp".to_string()
        }
    }
}

#[test]
fn main() {
    assert_eq!(doom::map(1, 1), "e1m1");
    assert_eq!(doom::creature::creature_name(), "imp");
}
