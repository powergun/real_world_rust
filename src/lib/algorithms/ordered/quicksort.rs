#[allow(dead_code)]
fn quick_sort<T: PartialOrd>(xs: &mut Vec<T>) {
    // directly copied from the C++ impl
    fn f<T: PartialOrd>(xs: &mut Vec<T>, first: usize, last: usize) {
        if last - first > 1 {
            let mut p = last;
            for i in first + 1..last {
                if xs[i] > xs[first]{
                    p -= 1;
                    xs.swap(i, p);
                }
            }
            p -= 1;
            xs.swap( p, first);
            f(xs, first, p);
            f(xs, p + 1, last);
        }
    }
    f(xs, 0, xs.len());
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