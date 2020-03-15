use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<Node>>;
type Link = Option<NodeType>;

pub struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(_s: String) -> NodeType {
        Rc::new(
            RefCell::new(
                Node { value: _s, next: None, prev: None }
            )
        )
    }
}

pub struct TransactionLog {
    _head: Link,
    _tail: Link,
    _length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { _head: None, _tail: None, _length: 0 }
    }

    pub fn head(&self) -> Link {
        self._head.clone()
    }

    pub fn tail(&self) -> Link {
        self._tail.clone()
    }

    pub fn length(&self) -> u64 {
        self._length
    }

    pub fn append(&mut self, _s : String) {
        let new_node = Node::new(_s);
        match self._tail.take() {
            None => {
                self._head = Some(new_node.clone());
            },
            Some(ref old_tail) => {
                new_node.borrow_mut().prev = Some(old_tail.clone());
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
        };
        self._length += 1;
        self._tail = Some(new_node.clone());
    }
}

// algorithms with rust L1555
// looking at the list without consuming it is an iterator's job
// implement the trait
pub struct LogIterator {
    _current: Link,
}

impl LogIterator {
    pub fn new(start_at: Link) -> LogIterator {
        LogIterator { _current: start_at }
    }

    pub fn current(&self) -> Link {
        self._current.clone()
    }
}

impl Iterator for LogIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current_link = self._current.clone();
        let mut result = None;
        self._current = match current_link {
            Some(ref current_cell) => {
                let current_ref = current_cell.borrow();
                result = Some(current_ref.value.clone());
                current_ref.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoubleEndedIterator for LogIterator {
    fn next_back(&mut self) -> Option<String> {
        let current_link = self._current.clone();
        let mut result = None;
        self._current = match current_link {
            Some(ref current_cell) => {
                // immut ref
                let current_ref = current_cell.borrow();
                // clone the string to be returned
                result = Some(current_ref.value.clone());
                // don't want to pass the ownership from the 
                // node to the iter, hence giving it a clone
                current_ref.prev.clone()
            },
            None => None
        };
        result
    }
}
