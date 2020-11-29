#[allow(unused_imports)]
use std::sync::mpsc::{channel, Receiver, Sender};
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
use std::thread::{spawn, JoinHandle};

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}

#[allow(dead_code)]
type Job = Box<dyn FnBox + 'static + Send>;

#[allow(dead_code)]
struct ThreadPool {
    handles: Vec<JoinHandle<()>>,
    ch: Sender<Job>,
}

impl ThreadPool {
    #[allow(dead_code)]
    fn new(n: usize) -> Self {
        let (ch_send, ch_recv) = channel::<Job>();
        let job_recv = Arc::new(Mutex::new(ch_recv));
        let mut handles = Vec::with_capacity(n);
        for _ in 0..n {
            let job_recv_th = job_recv.clone();
            handles.push(spawn(move || {
                loop {
                    // find job
                    let job = match job_recv_th.lock().unwrap().recv() {
                        Ok(v) => v,
                        _ => return,
                    };
                    job.call_box();
                }
            }));
        }
        ThreadPool {
            handles: handles,
            ch: ch_send,
        }
    }

    #[allow(dead_code)]
    fn add<F: FnOnce() + 'static + Send>(&self, f: F) {
        self.ch.send(Box::new(f)).unwrap();
    }

    // consume self and kill it
    #[allow(dead_code)]
    fn end(self) {
        drop(self.ch);
        for h in self.handles {
            h.join().expect("can not join");
        }
    }
}

#[test]
fn demo_use_thread_pool() {
    let p = ThreadPool::new(10);
    p.add(|| {
        println!("@@@ {}", 1);
    });
    p.end();
}
