// 225. Implement Stack using Queues
//     Easy

//     Implement the following operations of a stack using queues.

//     push(x) -- Push element x onto stack.
//     pop() -- Removes the element on top of the stack.
//     top() -- Get the top element.
//     empty() -- Return whether the stack is empty.

//     Example:

// MyStack stack = new MyStack();

// stack.push(1);
// stack.push(2);  
// stack.top();   // returns 2
// stack.pop();   // returns 2
// stack.empty(); // returns false

use std::mem;
use std::collections::VecDeque;

pub struct MyStack {
    // use VecDeques as queues
    first: VecDeque<i32>,
    second: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        return MyStack {
            first: VecDeque::new(),
            second: VecDeque::new(),
        }
    }

    /** Push element x onto the stack. */
    pub fn push(&mut self, x: i32) {
        self.first.push_back(x);
    }

    /** Removes the element on top of the stack and returns that element **/
    pub fn pop(&mut self) -> i32 {
        // Move everything except the last element in first to second
        // E.g. start position: first [1, 2, 3] and second []
        // first is now => [3], second is [1, 2]
        // Return 3, and first is now [1, 2] and second is []

        if !MyStack::empty(self) {
            while self.first.len() > 1 {
                if let Some(x) = self.first.pop_front() {
                    self.second.push_back(x);
                }
            }
            
            let value = self.first.pop_front().unwrap();
            mem::swap(&mut self.first, &mut self.second);
            value
        } else {
            i32::MIN
        }
    }

    /** Get the top element. **/
    pub fn top(&self) -> i32 {
        if !MyStack::empty(self) {
            self.first[self.first.len() - 1]
        } else {
            i32::MIN
        }
    }

    /** Returns whether the stack is empty. */
    pub fn empty(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // stack.push(1);
    // stack.push(2);  
    // stack.top();   // returns 2
    // stack.pop();   // returns 2
    // stack.empty(); // returns false


    #[test]
    pub fn test_0225() {
        let mut s = MyStack::new();
        s.push(1);
        s.push(2);
        // [1, 2]  []
        assert_eq!(2, s.top());
        // [1, 2] [] => [2] [1] => [] [1] => [1] []
        assert_eq!(2, s.pop());
        // [1] []
        assert_eq!(false, s.empty());
        // [1] []
        assert_eq!(1, s.pop());
        // [] []
        assert_eq!(true, s.empty());
        assert_eq!(i32::MIN, s.top());
        assert_eq!(i32::MIN, s.pop());
    }
}
