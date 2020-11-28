#[allow(unused_imports)]
use std::process::Command;

#[test]
fn call_other_program_or_panic() {
    let c = Command::new("w")
        .arg("--short")
        .output()
        .expect("w is not available");
    let s = String::from_utf8(c.stdout).expect("Not utf8 encoding");
    assert_eq!(s.len() > 0, true);
}

// to fail gracefully I need to define my own error type and
// aggregate these two levels of the error (execution error and
// utf8 decoding err)
#[test]
fn call_other_program_fail_silently() {
    let cmd_result = Command::new("wwww").arg("--short").output();
    let output = match cmd_result {
        Ok(handle) => match String::from_utf8(handle.stdout) {
            Ok(text) => text,
            _ => "".to_string(),
        },
        _ => "".to_string(),
    };
    assert_eq!(output, "".to_string());
}
