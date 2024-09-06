use std::collections::VecDeque;

struct MyQueue {
    q: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { q: VecDeque::new() }
    }

    fn push(&mut self, x: i32) {
        self.q.push_back(x);
        println!("push: {:?}", self.q);
    }

    fn pop(&mut self) -> i32 {
        println!("pop: {:?}", self.q);
        self.q.pop_front().unwrap_or_else(|| 0)
    }

    fn peek(&self) -> i32 {
        let x = self.q.front();

        println!("peek q: {:?}", self.q);
        println!("peek return : {:?}", x);
        if x.is_some() {
            return x.unwrap().clone();
        } else {
            return 0;
        }
    }

    fn empty(&self) -> bool {
        return self.q.is_empty();
    }
}
pub fn solve() -> u32 {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    let ret_3: i32 = obj.peek();
    let ret_2: i32 = obj.pop();
    let ret_4: bool = obj.empty();
    println!("{:?},{:?},{:?}", ret_2, ret_3, ret_4);
    0
}
