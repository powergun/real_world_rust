// source
// https://www.udemy.com/rust-building-reusable-code-with-rust-from-scratch/learn/lecture/13316098#overview

struct CreStore {
    count: i32,
}

impl Iterator for CreStore {
    type Item = i32;
    fn next(&mut self) -> std::option::Option<i32> {
        if self.count < 10 {
            let v = self.count;
            self.count += 1;
            return Some(v);
        } else {
            return None;
        }
    }
}

#[test]
fn demo_all() {
    let mut store = CreStore { count: 6 };
    while let Some(v) = store.next() {
        println!("{}", v);
    }
    // store continues to return None
}
