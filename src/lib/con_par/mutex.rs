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

#[test]
fn demo_mutex() {
    // rust enforces mutex locking by requesting the data source
    // be wrapped in Arc & Mutex
    let store_with_mutex = Arc::new(Mutex::new(Vec::<i32>::new()));
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
        }));
    }

    // consume and join all threads
    handles
        .into_iter()
        .for_each(|hd| hd.join().expect("fail to join"));

    let store = store_with_mutex.lock().unwrap();
    assert_eq!(store.len(), 10);
}
