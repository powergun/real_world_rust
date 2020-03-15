use std::thread;
// algorithms with rust L776
// in order to provide that ownership across threads - similar
// to what Rc does within a single thread - rust provides the
// concept of an Arc, an atomic reference counter.
// Using this mutex on top, it is the thread-safe equivalent
// of an Rc wrapping a RefCell, a reference counter that wraps
// a mutable container.
use std::sync::{Arc, Mutex};

pub fn demo_mutex() {
    let v = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vector = numbers.lock().unwrap();
            (*vector).push(i);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", *v.lock().unwrap());
}

#[test]
fn demo_all() {
    demo_mutex();
}
