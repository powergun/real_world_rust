
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
        Rc::new(RefCell::new(Node { value: s.clone(), next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, s : String) {
        let new_node = Node::new(s);
        match self.tail.take() {
            // has tail
            Some(ref old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            },
            // no tail
            None => {
                // and if head is None, both head and tail will
                // point to (via Rc) the same RefCell of the first 
                // node after this round 
                // there is no need to test the none-ness
                // i.e. the case where head is not-None but 
                // tail is None will not happen
                self.head = Some(new_node.clone());
            }
        };
        self.length += 1;
        self.tail = Some(new_node.clone());
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            if let Some(ref current_head) = old_head.borrow_mut().next {
                // chop head (head = head.next)
                self.head = Some(current_head.clone());
            }
            else {
                // no head, chop tail
                self.tail = None;
            }
            self.length -= 1;
            Rc::try_unwrap(old_head).ok().expect("///failed///")
                .into_inner().value
        })
    }

    pub fn drop(&mut self) {
        while ! self.tail.is_none() {
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
        let mut result = None;
        match self.current.take() {
            None => {},
            Some(ref old_current_ref) => {
                result = Some(old_current_ref.borrow().value.clone());
                if let Some(ref old_next_ref) = old_current_ref.borrow().next {
                    self.current = Some(old_next_ref.clone());
                }
            }
        };
        result
    }
}