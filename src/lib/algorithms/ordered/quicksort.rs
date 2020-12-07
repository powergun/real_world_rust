#[allow(unused_imports)]
use rand::Rng;
use std::fmt::Debug;

#[allow(dead_code)]
fn quick_sort<T: PartialOrd + Copy>(xs: &mut Vec<T>) {
    // directly copied from the C++ impl
    fn f<T: PartialOrd + Copy>(xs: &mut Vec<T>, first: usize, last: usize) {
        if last - first > 1 {
            let mut p = last;
            for i in first + 1..last {
                if xs[i] > xs[first] {
                    p -= 1;
                    xs.swap(i, p);
                }
            }
            p -= 1;
            xs.swap(p, first);
            f(xs, first, p);
            f(xs, p + 1, last);
        }
    }
    f(xs, 0, xs.len());
}

#[allow(dead_code)]
fn quick_sort_rand<T: PartialOrd + Debug>(xs: &mut Vec<T>) {
    // use a pseudo-random generator to pick up the pivot randomly to
    // improve the performance.

    let mut rng = rand::thread_rng();
    // rng.gen_range(low, high) to produce a number within range [low, high)

    fn f<T: PartialOrd + Debug>(xs: &mut Vec<T>, first: usize, last: usize, gen: &mut rand::rngs::ThreadRng) {
        if last - first > 1 {
            xs.swap(first, last - 1);
            let mut p = last;
            for i in first + 1..last {
                if xs[i] > xs[first] {
                    p -= 1;
                    xs.swap(i, p);
                }
            }
            p -= 1;
            xs.swap(p, first);
            f(xs, first, p, gen);
            f(xs, p + 1, last, gen);
        }
    }

    f(xs, 0, xs.len(), &mut rng);
}

#[test]
fn demo_quick_sort() {
    {
        let mut xs: Vec<_> = (1..13).rev().collect();
        quick_sort(&mut xs);
        assert_eq!((1..13).collect::<Vec<i32>>(), xs);
    }
    {
        let mut xs: Vec<i32> = vec![];
        quick_sort(&mut xs);
        assert!(xs.is_empty());
    }
    {
        let mut xs: Vec<i32> = vec![1];
        quick_sort(&mut xs);
        assert_eq!(vec![1], xs);
    }
    {
        let mut xs: Vec<i32> = vec![3, 1, 4];
        quick_sort(&mut xs);
        assert_eq!(vec![1, 3, 4], xs);
    }
}

#[test]
fn demo_quick_sort_random_pivot() {
    {
        let mut xs: Vec<_> = (1..13).rev().collect();
        quick_sort_rand(&mut xs);
        assert_eq!((1..13).collect::<Vec<i32>>(), xs);
    }
    {
        let mut xs: Vec<i32> = vec![];
        quick_sort_rand(&mut xs);
        assert!(xs.is_empty());
    }
    {
        let mut xs: Vec<i32> = vec![1];
        quick_sort_rand(&mut xs);
        assert_eq!(vec![1], xs);
    }
    {
        let mut xs: Vec<i32> = vec![3, 1, 4];
        quick_sort_rand(&mut xs);
        assert_eq!(vec![1, 3, 4], xs);
    }
}

#[test]
fn performance_comparison() {
    // 10 loops: 50.7381604 ms
    // 10 loops: 35.7711494 ms
    // however when workload increases to 10000 it causes stackoverflow
    {
        let mut xs: Vec<_> = (1..1000).rev().collect();
        timeit!({
            quick_sort(&mut xs);
        });
    }
    {
        let mut xs: Vec<_> = (1..1000).rev().collect();
        timeit!({
            quick_sort_rand(&mut xs);
        });
    }
}