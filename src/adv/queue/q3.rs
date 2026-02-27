struct MyQueue {
    left: Vec<i32>,
    right: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.right.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.transfer();

        self.left.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.transfer();

        *self.left.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }

    fn transfer(&mut self) {
        if self.left.is_empty() {
            while !self.right.is_empty() {
                self.left.push(self.right.pop().unwrap());
            }
        }
    }
}