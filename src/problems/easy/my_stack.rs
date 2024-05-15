struct MyStack {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap()
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        matches!(self.stack.len(), 0)
    }
}
pub fn solve() {
    let x = 8;
    let mut obj = MyStack::new();
    obj.push(x);
    let ret_2: i32 = obj.top();
    let ret_3: i32 = obj.pop();
    let ret_4: bool = obj.empty();

    assert_eq!(ret_2, 8);
    assert_eq!(ret_3, 8);
    assert!(ret_4);
}
