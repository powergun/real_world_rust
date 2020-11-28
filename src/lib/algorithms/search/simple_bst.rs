// a simple binary sort tree using Boxed type
// recall the similar example in Haskell using recursive type

#[derive(Debug)]
#[allow(dead_code)]
enum BST<T> {
    Empty,
    Tree(T, Box<BST<T>>, Box<BST<T>>),
}

impl<T> BST<T>
where
    T: PartialOrd,
    T: Clone,
{
    #[allow(dead_code)]
    fn empty() -> Self {
        Self::Empty
    }

    #[allow(dead_code)]
    fn singleton(x: T) -> Self {
        Self::Tree(x, Box::new(Self::empty()), Box::new(Self::empty()))
    }

    // consuming self
    #[allow(dead_code)]
    fn insert(self, x: T) -> Self {
        match self {
            Self::Empty => Self::singleton(x),
            Self::Tree(v, l, r) => {
                if x > v {
                    BST::Tree(v, l, Box::new(r.insert(x)))
                } else if x == v {
                    BST::Tree(v, l, r)
                } else {
                    BST::Tree(v, Box::new(l.insert(x)), r)
                }
            }
        }
    }

    // mutating self
    #[allow(dead_code)]
    fn m_insert(&mut self, x: T) {
        match self {
            Self::Empty => *self = Self::singleton(x),
            Self::Tree(v, l, r) => {
                if x > *v {
                    r.m_insert(x)
                } else if x < *v {
                    l.m_insert(x)
                }
            }
        }
    }

    #[allow(dead_code)]
    fn inorder_f(&self, f: fn(&T) -> ()) -> () {
        match self {
            Self::Empty => (),
            Self::Tree(v, l, r) => {
                l.inorder_f(f);
                f(v);
                r.inorder_f(f);
            }
        }
    }

    #[allow(dead_code)]
    fn inorder_vec(&self) -> Vec<T> {
        match self {
            Self::Empty => vec![],
            Self::Tree(v, l, r) => {
                let mut lv = l.inorder_vec();
                lv.push(v.clone());
                lv.extend(r.inorder_vec());
                lv
            }
        }
    }
}

#[test]
fn test_populate_bst() {
    {
        let root: BST<i32> = BST::singleton(10);
        let m = root.insert(12).insert(3).insert(5);
        m.inorder_f(|x: &i32| print!("{} ", x));

        let v = m.inorder_vec();
        assert_eq!(v, vec![3, 5, 10, 12]);
    }
    println!("");
    {
        let mut root: BST<i32> = BST::singleton(10);
        vec![5, 3, 12].into_iter().for_each(|x| root.m_insert(x));
        root.inorder_f(|x: &i32| print!("{} ", x));

        let v = root.inorder_vec();
        assert_eq!(v, vec![3, 5, 10, 12]);
    }
    println!("");
}
