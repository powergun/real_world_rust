//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin;
//$(which mkdir) -p ${dst};
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

// the content lives in doomlib/mod.rs OR doomlib.rs
#[allow(unused_imports)]
use super::doomlib;

// prefer folder structure for nested modules
/*
.
├── doomlib  <-- mod doomlib;
│   ├── creaturelib.rs  <-- private mod sourced by mod.rs
│   └── mod.rs  <-- doomlib's entry point; syms are exposed here
├── folder_structure.rs <-- user of doomlib
*/

#[test]
fn demo() {
    assert_eq!(doomlib::map(3, 3), "e3m3");
    assert_eq!(doomlib::creaturelib::creature_name(), "imp");
}
