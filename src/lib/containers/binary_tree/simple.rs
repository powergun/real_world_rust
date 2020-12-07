
#[allow(dead_code)]
enum BinTree<T> {
    Empty,
    BinTree {
        value: T,
        left: Box<BinTree<T>>,
        right: Box<BinTree<T>>,
    }
}

#[allow(dead_code)]
impl<T: PartialOrd> BinTree<T> {
    fn empty() -> Self {
        BinTree::Empty
    }

    fn singleton(x: T) -> Self {
        BinTree::BinTree {
            value: x,
            left: Box::new(BinTree::Empty),
            right: Box::new(BinTree::Empty),
        }
    }

    fn insert(&mut self, x: T) {
        match self {
            Self::Empty => *self = Self::singleton(x),
            Self::BinTree {value, left, right} => {
                if x > *value {
                    right.insert(x)
                } else if x < *value {
                    left.insert(x)
                }
            },
        }
    }

    fn traverse(&self, f: fn(&T) -> ()) {
        match self {
            Self::Empty => (),
            Self::BinTree {value, left, right} => {
                left.traverse(f);
                f(value);
                right.traverse(f);
            },
        }
    }
}

#[test]
fn demo_bin_tree_creation() {
    let mut root: BinTree<i32> = BinTree::empty();
    let ranges = vec![(1..4), (-3..-1), (5..7)];
    for range in ranges.into_iter() {
        for x in range {
            root.insert(x);
        }
    }
    root.traverse(|x: &i32| {
        println!("{}", x);
    });
}
