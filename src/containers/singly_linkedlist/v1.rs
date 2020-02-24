use std::cell::RefCell;
use std::rc::Rc;

// algorithms with rust L1482
type NodeType = Rc<RefCell<Node>>;
type SingleLink = Option<NodeType>;

#[derive(Debug)]
pub struct Node {
    pub value: String,
    // heap alloc, pointer size, let the compiler to decide
    // the size of this struct
    pub next: SingleLink,
}

impl Node {
    pub fn new(value: String) -> NodeType {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct TransactionLog {
    pub head: SingleLink,
    pub tail: SingleLink,
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    // take advantage of Option.take(), and use match statement
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.take
    // Takes the value out of the option, leaving a None in its place.
    // see also: https://doc.rust-lang.org/stable/std/mem/fn.replace.html
    //
    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            // Some(old), old is of NodeType, which is Rc<RefCell<T>>
            // must call borrow_mut() to create short-living mut ref
            // can also use ref pattern here, old is an immut
            // ref to Rc<RefCell<T>>; it does not bring a big benefit
            Some(old) => old.borrow_mut().next = Some(new_node.clone()),

            // self.head is of Option type therefore can directly
            // take Some(n)
            None => self.head = Some(new_node.clone()),
        };
        self.length += 1;
        self.tail = Some(new_node);
    }

    // example from book L1482
    pub fn pop(&mut self) -> Option<String> {
        // map will naturally stop when head is None
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                // chop the head
                self.head = Some(next);
            } else {
                // no head anymore, chop the tail
                self.tail = None;
            }
            // if map() stops calling the inner block, length
            // won't be decremented any more
            self.length -= 1;

            // piping the String out
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner() // consumes the refcell
                .value // returns an option!
        })
    }

    pub fn drop(&mut self) {
        while !self.tail.is_none() {
            self.pop();
        }
    }
}

pub struct LogIterator {
    current: SingleLink,
}

impl LogIterator {
    pub fn new(start_at: SingleLink) -> LogIterator {
        LogIterator { current: start_at }
    }
}

impl Iterator for LogIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current_link = self.current.clone();
        let mut result = None;
        self.current = match current_link {
            // I can also use value semantics here, instead of 
            // using an immut ref which saves a call to clone
            // the Rc; but isn't clone() the suggested approach?
            Some(current_ptr) => {
                let current_node = current_ptr.borrow();
                result = Some(current_node.value.clone());
                current_node.next.clone()
            },
            None => None
        };
        result
    }
}