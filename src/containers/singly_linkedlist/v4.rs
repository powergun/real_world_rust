
use std::rc::Rc;
use std::cell::RefCell;

type NodeType = Rc<RefCell<Node>>;
type Link = Option<NodeType>;

pub struct Node {
    pub value : String,
    pub next : Link,
}

pub struct TransactionLog {
    pub head : Link,
    pub tail : Link,
    pub length : u64,
}

pub struct LogIterator {
    current : Link,
}

impl Node {
    pub fn new(s : String) -> NodeType {
        Rc::new(RefCell::new(Node { value: s, next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, s : String) {
        let new_node = Node::new(s);
        match self.tail.take() {
            None => {
                self.head = Some(new_node.clone());
            },
            Some(ref old_tail_ref) => {
                old_tail_ref.borrow_mut().next =
                    Some(new_node.clone());
            }
        };
        self.length += 1;
        self.tail = Some(new_node.clone());
    }

    pub fn pop(&mut self) -> Option<String> {
        let result = self.head.take().map(|old_head| {
            match old_head.borrow().next {
                // chop the head
                Some(ref new_head_ref) => {
                    self.head = Some(new_head_ref.clone());
                },
                // chop the tail
                None => {
                    self.tail = None;
                }
            }
            self.length -= 1;
            Rc::try_unwrap(old_head).ok().expect("failed")
                .into_inner().value
        });
        result
    }

    pub fn drop(&mut self) {
        while ! self.tail.is_none() {
            self.pop();
        }
    }
}

impl LogIterator {
    pub fn new(start_up : Link) -> LogIterator{
        LogIterator { current: start_up }
    }
}

impl Iterator for LogIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut result = None;
        match self.current.take() {
            None => {},
            Some(ref old_current) => {
                result = Some(old_current.borrow().value.clone());
                self.current = old_current.borrow().next.clone();
            },
        };
        result
    }
}