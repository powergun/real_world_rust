// FP in rust L594

// data trait (OOP)
pub trait DataTrait {
    // ctor
    fn new(a: i32, b: String) -> Self;

    // meth
    fn get_value(&self) -> i32;
    fn to_str(&self) -> String;
}

// typeclass-like trait (FP)
pub trait BehaviorTrait {
    fn bark(&self) -> String;
}

// implement trait on a type
// https://doc.rust-lang.org/book/ch10-02-traits.html
pub struct Node {
    // private fields
    label: String,
    init: i32,
}

// trait provides the OO interface
impl DataTrait for Node {
    fn new(a: i32, b: String) -> Self {
        Node { label: b, init: a }
    }

    fn get_value(&self) -> i32 {
        self.init * 10
    }
    fn to_str(&self) -> String {
        format!("[{}]", &self.label)
    }
}

impl BehaviorTrait for Node {
    fn bark(&self) -> String {
        format!("{:?} >-> {}", &self.init, &self.label)
    }
}

impl BehaviorTrait for i32 {
    fn bark(&self) -> String {
        format!("{:?} >-> nop", self)
    }
}

#[test]
pub fn test_data_trait() {
    let n = Node::new(10, "e1m1".to_string());
    assert_eq!(100, n.get_value());
    assert_eq!("[e1m1]", n.to_str());
}

#[test]
pub fn test_behavior_trait() {
    let n = Node::new(101, "iddqd".to_string());
    let i: i32 = 102;
    assert_eq!("101 >-> iddqd", n.bark());
    assert_eq!("102 >-> nop", i.bark());
}

// I can then use the behavior trait to write generic algorithms
// trait bound (haskell's type constraint)
// see: https://doc.rust-lang.org/1.4.0/book/traits.html
pub fn do_bark<T>(xs: Vec<T>) -> String
where
    T: BehaviorTrait,
{
    xs.iter().fold(String::new(), |acc, elem| {
        if acc.is_empty() {
            elem.bark()
        } else {
            format!("{}, {}", acc, elem.bark())
        }
    })
}

#[test]
pub fn test_trait_bound() {
    let nodes = vec![
        Node::new(101, "iddqd".to_string()),
        Node::new(-1, "deadbeef".to_string()),
    ];
    let nums: Vec<i32> = vec![102, -1];
    assert_eq!("101 >-> iddqd, -1 >-> deadbeef", do_bark(nodes));
    assert_eq!("102 >-> nop, -1 >-> nop", do_bark(nums));
}
