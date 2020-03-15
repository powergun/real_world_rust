
use std::cell::RefCell;
use std::rc::Rc;

type NodeType = Rc<RefCell<Node>>;
type Link = Option<NodeType>;

#[derive(Clone)]
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
        // None, None
        //   | / 
        //  []
        //  []   [] 
        //  []   []   [] ...
        let new_node = Node::new(s);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            },
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.length += 1;
        self.tail = Some(new_node.clone());
    }

    // popleft()
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            if let Some(ref old_next) = old_head.borrow_mut().next {
                self.head = Some(old_next.clone());
            }
            else {
                self.tail = None;
            }
            self.length -= 1;
            Rc::try_unwrap(old_head)
                .ok().expect("fatal error").into_inner().value
        })
    }

    pub fn drop(&mut self) {
        while !self.tail.is_none() {
            self.pop();
        }
    }
}

impl LogIterator {
    pub fn new(start_at : Link) -> LogIterator {
        LogIterator { current: start_at }
    }
}

impl Iterator for LogIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current_link = self.current.clone();
        let mut result = None;
        self.current = match current_link {
            Some(current_ref) => {
                let current_node = current_ref.borrow();
                result = Some(current_node.value.clone());
                current_node.next.clone()
            },
            None => None
        };
        result
    }
}