mod v1;
mod simple;

pub use v1::*;

#[test]
fn test_node_creation() {}

#[test]
fn test_list_append() {
    let mut l = TransactionLog::new_empty();
    assert_eq!(0, l.length());
    (0..5).for_each(|idx| l.append(format!("AA_{:02X}", idx)));
    let actual: Vec<String> = LogIterator::new(l.head())
        .map(|elem| format!("{}", elem))
        .collect();
    assert_eq!(5, l.length());
    assert_eq!("AA_00,AA_01,AA_02,AA_03,AA_04", actual.join(","));
}

#[test]
fn test_list_iterator_reverse() {
    let mut l = TransactionLog::new_empty();
    (0..5).for_each(|idx| l.append(format!("AA_{:02X}", idx)));
    let actual: Vec<String> = LogIterator::new(l.tail())
        .rev()
        .map(|elem| format!("{}", elem))
        .collect();
    assert_eq!("AA_04,AA_03,AA_02,AA_01,AA_00", actual.join(","));
}

#[test]
fn test_list_iterator_bidirectional_traverse() {
    let mut l = TransactionLog::new_empty();
    (0..10).for_each(|idx| l.append(format!("AA_{:02X}", idx)));
    let mut it = LogIterator::new(l.head());
    let mut actual: Vec<String>; //= Vec::new(); // not necessary (warning: no used)

    actual = (0..3).map(|_| format!("{}", it.next().unwrap())).collect();
    assert_eq!("AA_00,AA_01,AA_02", actual.join(","));

    // current points to AA_03 (the last value yield is AA_02)

    (0..6).for_each(|_| {
        it.next();
    });

    // current points to AA_09 (the last value yield is AA_08)

    actual = it.rev().take(5).map(|elem| format!("{}", elem)).collect();
    assert_eq!("AA_09,AA_08,AA_07,AA_06,AA_05", actual.join(","));
}

#[test]
fn test_all() {
    test_node_creation();
    test_list_append();
    test_list_iterator_reverse();
    test_list_iterator_bidirectional_traverse();
}
