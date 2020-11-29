#[allow(unused_imports)]
use std::thread::{sleep, spawn};
#[allow(unused_imports)]
use std::time::Duration;

#[test]
fn demo_sleep() {
    sleep(Duration::from_micros(300));
}
