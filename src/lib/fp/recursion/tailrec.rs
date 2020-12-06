#[allow(dead_code)]
fn sum_all(xs: &[i32], acc: i32) -> i32 {
    if xs.is_empty() {
        acc
    } else {
        sum_all(&xs[1..xs.len()], xs.get(0).unwrap() + acc)
    }
}

#[test]
fn demo_tailrec_cause_stack_overflow() {
    // rust does not offer tail-call optimization, see:
    // https://dev.to/seanchen1991/the-story-of-tail-call-optimizations-in-rust-35hf
    // therefore it is much slower than the iterative approach and causes stack overflow when
    // the workload is sufficiently large
    for workload in vec![
        10, 20, 30
        // stack overflow
        // 1000, 10000, 100000, 1000000
    ] {
        let xs: Vec<_> = (1..workload).collect();
        timeit!({
            sum_all(&xs, 0);
        });
            timeit!({
            xs.iter().sum::<i32>();
        });
    }
}