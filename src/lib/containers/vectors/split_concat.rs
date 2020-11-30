// source
// rust std lib cookbook P/49

// xs -> hd::tail
// see also: xs.split_last()
#[test]
fn demo_split_first() {
    let xs = vec![1, 2, 3, 4, 5];

    if let Some((first, xs_)) = xs.split_first() {
        assert_eq!(first, &1);
        assert_eq!(4, xs_.len());
    } else {
        panic!("shall not fail!");
    }
}

#[test]
fn demo_split_at() {
    let xs = vec![1, 2, 3, 4, 5];
    {
        let (left, right) = xs.split_at(2);
        assert_eq!(vec![1, 2], left);
        assert_eq!(vec![3, 4, 5], right);
    }
    {
        // this method will Panics
        // Panics if mid > len.
        let (left, right) = xs.split_at(5);
        assert_eq!(vec![1, 2, 3, 4, 5], left);
        assert_eq!(0, right.len());
    }
}

#[test]
fn demo_mutating_split() {
    let mut xs = vec![1, 2, 3, 4, 5];
    let remaining = xs.split_off(3);

    // xs: [0, 3)
    assert_eq!(vec![1, 2, 3], xs);
    // remaining: [3, ..)
    assert_eq!(vec![4, 5], remaining);
}

#[test]
fn demo_mut_append() {
    let mut xs = vec![1, 2];
    let mut ys = vec![3, 4];

    // this empties ys and mutates xs
    xs.append(&mut ys);

    assert_eq!(0, ys.len());
    assert_eq!(vec![1, 2, 3, 4], xs);
}

#[test]
fn demo_split_at_mut() {
    // yields two mutable slices
    let mut xs = vec![1, 2, 3, 4];
    let pivot = xs.len() / 2;
    // will panic if pivot is out of bound
    let (left, right) = xs.split_at_mut(pivot);
    left[0] = 999;
    right[0] = 999;
    assert_eq!(xs, vec![999, 2, 999, 4]);
}
