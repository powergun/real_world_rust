use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub fn demo_channels() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..10).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            // don't use the result
            let _ = _tx.send(i).unwrap();
        })
    });

    // close all threads
    for h in handles {
        h.join().unwrap();
    }

    // receive N times
    let numbers: Vec<i32> = (0..10).map(|_| rx.recv().unwrap()).collect();

    println!("{:?}", numbers);
}

#[test]
fn demo_all() {
    demo_channels();
}
