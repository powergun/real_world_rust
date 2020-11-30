#[allow(unused_imports)]
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn demo_get_time_as_int_in_milliseconds() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let d: u128 = since_the_epoch.as_micros();
    println!("{:?}, d: {}", since_the_epoch, d);
}