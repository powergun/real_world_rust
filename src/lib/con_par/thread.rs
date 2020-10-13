// algorithms with rust L674
// concurrency and parallelism are two different modes of
// execution
// while concurrency means that parts of a program run
// independently of each other, parallelism refers to these
// parts executing at the same time

#[allow(unused_imports)]
use std::thread;

// || is the space where parameters go
// akin to a function signature's parameters, without the need
// to always declare types explicitly
// this way variables can move from the outer into the inner scope
#[test]
fn demo_thread_creation_join() {
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

// source
// rust std lib cookbook P/26

#[test]
fn demo_threaded_lambda() {
    let child = thread::spawn(|| println!("threaded lambda"));
    println!("main thread");
    child.join().expect("fail to join!");
}

#[test]
fn demo_parallel_sum() {
    let sum_bucket = |range: &[i32]| -> i32 {
        let mut sum = 0;
        for num in range {
            sum += *num;
        }
        sum
    };

    // sum the numbers in a slice in parallel
    let do_sum = |range: &[i32]| -> i32 {
        // how to get the number of cores dynamically
        // https://stackoverflow.com/questions/22155130/determine-number-of-cores-using-rust
        // using an external crate
        const NUM_THREADS: usize = 4;
        if range.len() < NUM_THREADS {
            // the tail case
            sum_bucket(range)
        } else {
            // define the per-thread workload size
            let bucket_size = range.len() / NUM_THREADS;
            let mut count = 0;
            let mut threads = Vec::new();
            while count + bucket_size < range.len() {
                // for data parallelism, bucket is a copy of
                // the original elements (the compiler enforces
                // it)
                let bucket = range[count..count + bucket_size].to_vec();
                // P27
                // thread::spawn() returns a JoinHandle
                // note that the return value (and type) is
                // derived from the pure function sum_bucket()
                let thread = thread::Builder::new()
                    .name("calc".to_string())
                    .spawn(move || sum_bucket(&bucket))
                    .expect("failed to create thread");
                threads.push(thread);
                count += bucket_size;
            }
            // if there are elements left over, they will be
            // processed outside the while loop
            let mut sum = sum_bucket(&range[count..]);
            for thread in threads {
                // note how calling join() and expect() yields
                // the return value from the wrapped function
                sum += thread.join().expect("failed to join");
            }
            sum
        }
    };

    assert_eq!(
        10 * 16,
        do_sum(&[
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1,
            2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2,
            3, 4, 1, 2, 3, 4,
        ])
    );
}
