// rust std lib cookbook P/67
// vec_deque has capacity
// internally vec deque uses a ring buffer

#[allow(unused_imports)]
use std::collections::VecDeque;

#[test]
fn demo_create_deque() {
    // vecdeque is a deque, which is FIFO
    let mut orders: VecDeque<String> = VecDeque::new();
    orders.push_back("value: T".to_string());
}

#[test]
fn demo_safe_pop_front() {
    let mut orders: VecDeque<String> = VecDeque::new();
    assert!(orders.pop_front().is_none());
}

#[test]
fn demo_swap_remove_both_ends() {
    let mut orders = VecDeque::new();
    (1..5).for_each(|x| orders.push_back(x)); // 1, 2, 3, 4
    orders.swap_remove_back(0);
    assert_eq!(4, orders[0]); // 4, 2, 3

    orders.swap_remove_front(1);
    assert_eq!(4, orders[0]); // 4, 3
}
