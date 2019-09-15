# two-sum

## 题目路径
https://leetcode-cn.com/problems/implement-queue-using-stacks/

## 题型分类
栈、队列

## 思路
设置两个栈，一个输入栈，一个输出栈；
在push时，将数据压入输入栈；
在pop或peek时，如果输出栈为空，则将输入栈中元素逐一取出压入输出栈；之后从输出栈取栈顶数据返回；
判空时当两个栈同时为空时返回空，否则为非空；

# 实现&用例
```
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

```

```
mod MyQueue;

#[cfg(test)]
mod tests {
    use crate::implement-queue-using-stacks::MyQueue;
    #[test]
    fn test_implement-queue-using-stacks() {
        let mq = MyQueue::new();
        mq.push(1);
        mq.push(2);
        assert_eq!(1, mq.pop());
        assert_eq!(false, mq.empty());
        assert_eq!(2, mq.pop());
        assert_eq!(true, mq.empty());
    }
}

```

## 复杂度分析
空间复杂度O(n)
时间复杂度：
push： O(1);
pop/peek: O(n);
empty: O(1);


## 结果
>执行用时 :0 ms, 在所有 Rust 提交中击败了100.00%的用户
内存消耗 :2 MB, 在所有 Rust 提交中击败了100.00%的用户