// source
// https://www.udemy.com/rust-building-reusable-code-with-rust-from-scratch/learn/lecture/13316098#overview

struct CreStore {
    count: i32,
}

impl Iterator for CreStore {
    type Item = i32;
    fn next(&mut self) -> std::option::Option<i32> {
        if self.count < 10 {
            let v = self.count;
            self.count += 1;
            return Some(v);
        } else {
            return None;
        }
    }
}

#[test]
fn demo_all() {
    let mut store = CreStore { count: 6 };
    while let Some(v) = store.next() {
        println!("{}", v);
    }
    // store continues to return None
}

// rust std lib cookbook P/80
// create a custom iterator
#[test]
fn demo_constant_iterator() {
    use std::collections::HashSet;

    struct A {
        val: i32,
    };

    // a custom iterator has to implement only one method:
    // next()
    // what comes next
    impl Iterator for A {
        type Item = i32;
        fn next(&mut self) -> Option<i32> {
            Some(self.val)
        }
    }
    let xs: HashSet<_> = A { val: 10 }.take(4).collect();
    assert_eq!(1, xs.len());
}

#[test]
fn demo_fibonacci_iterator() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    };

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            let old = self.curr;
            self.curr = self.next;
            self.next += old;
            Some(old)
        }
    };

    let fib = || -> Fibonacci { Fibonacci { curr: 0, next: 1 } };

    let xs: Vec<_> = fib().take(4).collect();
    assert_eq!(vec![0, 1, 1, 2], xs);
}

#[test]
fn demo_squares_iterator() {
    use std::ops::Deref;
    use std::ops::Mul;

    struct SquareVec<T>
    where
        T: Mul + Copy,
    {
        // T::Output is just the type that a multiplication will
        // return, which most of the time is going to be T itself
        vec: Vec<T::Output>,
    };

    impl<T> SquareVec<T>
    where
        T: Mul + Copy,
    {
        fn new() -> Self {
            SquareVec { vec: Vec::new() }
        }

        fn push(&mut self, item: T) {
            self.vec.push(item * item);
        }
    }

    // P/82
    // when creating an iterator over a collection-like struct
    // it is best to just allow it to be convertible into a
    // slice of your underlying type
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // this way you automatically implemented a bunch of methods
    // and are flexible enough to change your impl later on
    // this is an implicit conversion
    impl<T> Deref for SquareVec<T>
    where
        T: Mul + Copy,
    {
        type Target = [T::Output];
        fn deref(&self) -> &Self::Target {
            &self.vec
        }
    }

    let mut xs: SquareVec<i32> = SquareVec::new();
    (1..5).for_each(|x| xs.push(x));
    let xs_: Vec<_> = xs.iter().map(|&x| x).collect();
    //                  ^^^^ will deref before calling iter()
    //              iter() comes from the slice of T
    assert_eq!(vec![1, 4, 9, 16], xs_);
}
