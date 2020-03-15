// algorithms with rust L674
// concurrency and parallelism are two different modes of
// execution
// while concurrency means that parts of a program run
// independently of each other, parallelism refers to these
// parts executing at the same time

use std::thread;

// || is the space where parameters go
// akin to a function signature's parameters, without the need
// to always declare types explicitly
// this way variables can move from the outer into the inner scope
pub fn demo_thread_creation_join() {
    let _x = 10;

    let handle = thread::spawn(|| {
        // can not compile: may outlive borrowed value `_x
        // println!("threaded: {}", _x);

        // to fix: add move keyword
        // L750
        // however for passing multiple messages into a thread or
        // implementing an actor model, the rust standard library
        // offers channels

        println!("threaded");
        return 1334;
    });
    if let Ok(o) = handle.join() {
        println!("joined: {:?}", o); // <- return value from the subroutine
    }
}

#[test]
fn demo_all() {
    demo_thread_creation_join();
}
