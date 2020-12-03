#[allow(unused_imports)]
use std::thread::spawn;
// algorithms with rust L776
// in order to provide that ownership across threads - similar
// to what Rc does within a single thread - rust provides the
// concept of an Arc, an atomic reference counter.
// Using this mutex on top, it is the thread-safe equivalent
// of an Rc wrapping a RefCell, a reference counter that wraps
// a mutable container.
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};

// recall that in Scala, I can call synchronized {} on any
// non-primitive value and expect the code inside to be executed
// by a single thread - this is a time saver
// Rust on the other hand uses the scoped lock approach (similar to c++),
// but it enforces the caller to access only the wrapped data
// but nothing else

// unimplemented!() is similar to Scala's ???;
// the code will compile fine
fn _f() -> i32 {
    unimplemented!();
}

#[test]
fn demo_mutex() {
    // rust enforces mutex locking by requesting the data source
    // be wrapped in Arc & Mutex
    let mut _xs = Vec::<i32>::new();
    // this transfers the ownership to the block of mem to
    // the mutex; _xs can no longer be read;
    let store_with_mutex = Arc::new(Mutex::new(_xs));
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let write_lock = store_with_mutex.clone();
        let index = i; // can not move i directly to the thread
        handles.push(spawn(move || {
            // lock() is a method on Mutex but Arc (like RC)
            // allows caller to invoke method on the wrapped
            // type directly.
            // lock() returns a Result type;
            // the only time Err(e) can happen is when another
            // thread that holds the lock panics - it is called
            // mutex-poisoning;
            // unwrap() is more commonly used in this situation
            let mut p = write_lock.lock().unwrap();
            p.push(index);
            // if p were a primitive type, it will be a ref
        }));
    }

    // consume and join all threads
    handles
        .into_iter()
        .for_each(|hd| hd.join().expect("fail to join"));

    let store = store_with_mutex.lock().unwrap();
    assert_eq!(store.len(), 10);
    // _xs is gone; the only way to access the store is to
    // unwrap the mutex
    // assert_eq!(_xs.len(), 10);
}

#[test]
fn demo_try_lock() {
    // read: https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.try_lock
    let c = Arc::new(Mutex::new(0));
    match c.try_lock() {
        Ok(ref mut l) => **l = 1,
        _ => (),
    };
    assert_eq!(1, *c.lock().unwrap());
}

// lock can be poisoned - when a thread that holds the lock panics
