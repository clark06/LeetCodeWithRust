use std::collections::VecDeque;

struct MyQueue {
    inStack: VecDeque<i32>,
    outStack: VecDeque<i32>,
}

impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            inStack: VecDeque::new(),
            outStack: VecDeque::new(),
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.inStack.push_front(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.outStack.is_empty() {
            while let Some(x) = self.inStack.pop_front() {
                self.outStack.push_front(x)
            }
        }
        if let Some(ret) = self.outStack.pop_front() {
            return ret;
        }
        panic!("cannot pop");
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.outStack.is_empty() {
            while let Some(x) = self.inStack.pop_front() {
                self.outStack.push_front(x)
            }
        }
        if let Some(ret) = self.outStack.get_mut(0) {
            return *ret;
        }
        panic!("cannot peek");
    }

    /** Returns whether the queue is empty. */
    fn empty(&mut self) -> bool {
        self.outStack.is_empty() && self.inStack.is_empty()
    }
}