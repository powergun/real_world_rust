#[allow(unused_imports)]
use std::sync::mpsc::{channel, Receiver, Sender};
#[allow(unused_imports)]
use std::thread;

// mpsc stands for multiple producer single consumer (channel)

#[test]
fn demo_all() {
    let (producer, consumer): (Sender<i32>, Receiver<i32>) = channel();
    // or let (producer, consumer) = channel::<i32>();
    let handles = (0..10).map(|i| {
        // producer channel can be cloned freely, hence the
        // Multi-producer notion
        let producer_per_thread = producer.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = producer_per_thread.send(i).unwrap();
        })
    });

    // close all threads
    for h in handles {
        h.join().unwrap();
    }

    // receive N times; to handle the result of recv(), I
    // can either use unwrap() or pattern march (better)
    let numbers: Vec<i32> = (0..10)
        .map(|_| match consumer.recv() {
            Ok(v) => v,
            Err(_) => -1,
        })
        .collect();

    // how to close a send/producer channel?
    // the send channel closes when all the copies of the send
    // channel are "dropped" (i.e. they go out of scoped)
    // in the above code, the send channel inside the thread-
    // specific closure goes out of scope once execution is
    // complete;
    // or explicitly drop(producer) to the floor

    assert_eq!(numbers.len(), 10);
}
