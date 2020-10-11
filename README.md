# Real World Rust

[memory model](src/memory_model)

[data munging](src/data_munging)

[algorithms](src/algorithms)

[containers](src/containers)

[system programming](src/system_programming)

[performance](src/performance)

[concurrency & parallelism](src/con_par)

[debugging & black art](src/debugging)

## Project Structure and DevOps

install rustup; update rustup to the latest version; update rust
toolchain to the latest version;

`cargo build` to build the artifact; also `cargo test` is often
used;

install rust RLS in vscode and TOML language support; install prettier.

in vscode, create a singleton workspace containing only rust_real_world
repository (otherwise RLS won't work).

rust_real_world was created using the lib template:

`cargo new --lib rust_real_world`

### Lib target

see `[lib]` section, which is copied from the official doc page with
my modification;

the lib target is by default **seen** by the executable targets (the
binary targets i.e.) therefore I can write

`use lib_rw_rust::algorithms::iteration::filter::*;`

in the executable source file. see: `bin/echo.rs`

build the lib target using:

`cargo build`

or `cargo test` to execute all the `#[test]` test functions

### Bin target

NOTE: the target must be defined as `[[bin]]`

there are really two attributes, `name` and `path`, to define

execute the target using: `cargo run --bin <name>`; like haskell stack,
if there is only one binary target `cargo run` by default picks it
up.

### Linkage

see: <https://doc.rust-lang.org/nightly/reference/linkage.html>

I want to create big-fat binary for my executable target;

I want to enable sharing within the project (but not necessary
outside the project)

`rlib` is the one I need (see `[lib]` section); I can run
`cargo build --release` to create the release (opt) binary;

### References

<https://github.com/digikata/rust-lib-and-bin>

(defining multiple exe targets)

<https://doc.rust-lang.org/cargo/guide/project-layout.html>

<https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary>

(emphasizing the importance of naming the lib target, because
the exe target will use its name to import the functions)
