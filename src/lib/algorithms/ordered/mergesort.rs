
#[allow(dead_code)]
fn merge(xs: &[i32], ys: &[i32], o: &mut Vec<i32>) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let i_max: usize = xs.len();
    let j_max: usize = ys.len();
    while i < i_max && j < j_max {
        if xs[i] < ys[j] {
            o.push(xs[i]);
            i += 1;
        } else {
            o.push(ys[j]);
            j += 1;
        }
    }
    while i < i_max {
        o.push(xs[i]);
        i += 1;
    }
    while j < j_max {
        o.push(ys[j]);
        j += 1;
    }
}

#[allow(dead_code)]
fn merge_sort(xs: &[i32]) -> Vec<i32> {
    if xs.len() > 1 {
        let (a, b) = xs.split_at(xs.len() / 2);
        let av = merge_sort(a);
        let bv = merge_sort(b);
        let mut o: Vec<i32> = Vec::with_capacity(xs.len());
        merge(&av, &bv, &mut o);
        o
    } else {
        xs.to_vec()
    }
}

#[test]
fn test_merge_sorted_slices() {
    let xs: Vec<_> = vec![1, 2, 3, 4, 5];
    let ys: Vec<_> = vec![2, 4, 6, 7];
    {
        let mut o: Vec<i32> = Vec::with_capacity(xs.len() + ys.len());
        merge(&xs, &ys, &mut o);
        assert_eq!(vec![1, 2, 2, 3, 4, 4, 5, 6, 7], o);
    }
}

#[test]
fn test_merge_sort() {
    let xs: Vec<_> = (1..13).rev().collect();
    let ys = merge_sort(&xs);
    assert_eq!((1..13).collect::<Vec<i32>>(), ys);
}