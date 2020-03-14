pub struct TransactionLog {
    pub length: u64,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { length: 0 }
    }
}
pub struct Node {
    pub value: String,
}

impl Node {
    pub fn new(value: String) -> Node {
        Node { value: value }
    }
}
