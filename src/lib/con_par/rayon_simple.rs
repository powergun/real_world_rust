#[allow(unused_imports)]
use rayon::prelude::*;
#[allow(unused_imports)]
use std::thread::sleep;
#[allow(unused_imports)]
use std::time::Duration;

#[test]
fn demo_parallel_iterator_map() {
    // rayon under the hood uses join(A, B)
    // if A, B can run in parallel, do so; otherwise put B
    // in a queue and further calling to join(A, B) will keep
    // enqueuing tasks or stealing tasks from the queue
    
    // is it some form of work stealing?

    let xs = (0..10).collect::<Vec<i32>>();
    let mut ys = xs // change from into_par() to par_iter()
        .par_iter()
        .map(|x| {
            sleep(Duration::from_millis(100));
            println!("computing {}", x);
            x * x
        })
        .collect::<Vec<i32>>();
    assert_eq!(ys.len(), 10);

    // in-place modification
    ys.par_iter_mut().for_each(|x| *x = 1);
    assert_eq!(ys.len(), 10);
}
