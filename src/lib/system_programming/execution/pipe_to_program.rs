#[allow(unused_imports)]
use std::io::copy;
#[allow(unused_imports)]
use std::process::{Command, Stdio};

#[test]
fn demo_to_pipe() {
    // dest to write to
    let writer = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::piped())
        .spawn()
        .expect("fail to execute");
    // source to read from
    let reader = Command::new("cat")
        .arg("/etc/passwd")
        .stdout(Stdio::piped())
        .spawn()
        .expect("fail to execute cat");
    // copy(<from>, <to>)
    let result = copy(&mut reader.stdout.unwrap(), &mut writer.stdin.unwrap());
    assert_eq!(result.is_ok(), true);
    // println!("{:?}", result);
}
