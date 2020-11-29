#[allow(unused_imports)]
use std::process::{Command, Stdio};

#[test]
fn demo_to_pipe() {
    let c = Command::new("w")
        // .stdin(Stdio::piped)
        .spawn()
        .expect("fail to execute");
}
