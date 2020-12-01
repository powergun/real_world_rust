// FP in rust L586

// enum type in rust can take parameters and generic type params
pub enum BTree<T> {
    Node {
        val: T,
        left: Box<BTree<T>>,
        right: Box<BTree<T>>,
    },
    Leaf,
}

#[test]
pub fn test_enums() {
    let _l = BTree::Node {
        val: 1,
        left: Box::new(BTree::Leaf),
        right: Box::new(BTree::Leaf),
    };
    let _r = BTree::Node {
        val: 2,
        left: Box::new(BTree::Leaf),
        right: Box::new(BTree::Leaf),
    };
    let _node = BTree::Node {
        val: 5,
        left: Box::new(_l),
        right: Box::new(_r),
    };
    let _root = BTree::Node {
        val: 10,
        left: Box::new(BTree::Leaf),
        right: Box::new(_node),
    };

    // typecheck
    let _nodes: Vec<BTree<i32>> = vec![
        // the life time of these nodes have been owned by their
        // parents
        // _l, _r, _node,
        _root,
        BTree::Leaf,
    ];
}
