mod v4;
pub use v4::*;

#[test]
fn test_list_creation() {
    let translog = TransactionLog::new_empty();
    let head = Node::new("e1m1".to_string());
    assert_eq!(translog.length, 0);
    assert!(head.borrow().next.is_none());
}

#[test]
pub fn test_list_append() {
    let mut translog = TransactionLog::new_empty();
    // range is exclusive!!
    (1..5).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    assert_eq!(translog.length, 4);
}

#[test]
fn test_list_pop() {
    let mut translog = TransactionLog::new_empty();
    (0..50).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    assert_eq!(translog.length, 50);
    (0..5).for_each(|_| {
        if let Some(s) = translog.pop() {
            print!("popped: {}, ", s);
        }
    });
    println!("");
    assert_eq!(translog.length, 45);
    (0..50).for_each(|_| {
        translog.pop();
    });
    assert_eq!(translog.length, 0);
    assert!(translog.tail.is_none());
    assert!(translog.head.is_none());
}

#[test]
fn test_list_drop() {
    let mut translog = TransactionLog::new_empty();
    (0..50).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    // default drop() impl is recursive, so is Debug print trait
    // use a custom iterative impl
    translog.drop();
    assert_eq!(translog.length, 0);
    assert!(translog.tail.is_none());
    assert!(translog.head.is_none());
}

#[test]
fn test_log_iterator() {
    let mut translog = TransactionLog::new_empty();
    (0..10).for_each(|idx| {
        translog.append("AA_".to_string() + &idx.to_string());
    });
    let iter = LogIterator::new(translog.head);
    let mut idx = 0;
    iter.for_each(|s| {
        idx += 1;
        print!("{}, ", s);
    });
    println!("");
    assert_eq!(idx, 10);
}

#[test]
fn test_all() {
    test_list_creation();
    test_list_append();
    test_list_pop();
    test_list_drop();
    test_log_iterator();
}
