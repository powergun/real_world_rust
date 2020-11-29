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

    // process the text output
    // for (_, line) in s.split("\n").enumerate() {
    // }
    let n = s
        .split("\n")
        .into_iter()
        .filter(|line| line.len() > 0)
        .count();
    assert_eq!(n > 0, true);
}

// to fail gracefully I need to define my own error type and
// aggregate these two levels of the error (execution error and
// utf8 decoding err)
#[test]
fn call_other_program_or_fail_silently() {
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

#[test]
fn call_other_program_or_converge_errors() {
    type ResultT = Result<String, String>;

    let cmd_result = Command::new("wwww").arg("--short").output();
    let output: ResultT = match cmd_result {
        Ok(handle) => match String::from_utf8(handle.stdout) {
            Ok(text) => Ok(text),
            Err(details) => Err(format!("can not decode utf8: {}", details)),
        },
        Err(details) => Err(format!("can not execute wwww: {}", details)),
    };

    // converge all the internal errors (such as utf8decode error)
    // to a custom error type (ResultT); but lose the type info
    // as the error type is String
    assert_eq!(output.is_err(), true);
    // println!("{:?}", output);
}

#[test]
fn call_other_program_or_converge_to_own_types() {
    enum Error {
        Execution(String),
        Decode(String),
    };
    type ResultT = Result<String, Error>;

    let cmd_result = Command::new("wwww").arg("--short").output();
    let output: ResultT = match cmd_result {
        Ok(handle) => match String::from_utf8(handle.stdout) {
            Ok(text) => Ok(text),
            Err(details) => Err(Error::Execution(format!(
                "can not decode utf8: {}",
                details
            ))),
        },
        Err(details) => Err(Error::Decode(format!("can not execute wwww: {}", details))),
    };

    // converge all the internal errors (such as utf8decode error)
    // to a custom error type (ResultT); map the internal types
    // to my own types - preserving the type info to some extend
    assert_eq!(output.is_err(), true);

    // handle each individual error type
    match output {
        Err(Error::Execution(_details)) => (),
        Err(Error::Decode(_details)) => (),
        _ => (),
    }
}
