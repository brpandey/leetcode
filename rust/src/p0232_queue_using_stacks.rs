// 232. Implement Queue using Stacks

//     Implement the following operations of a queue using stacks.

//     push(x) -- Push element x to the back of queue.
//     pop() -- Removes the element from in front of queue.
//     peek() -- Get the front element.
//     empty() -- Return whether the queue is empty.

//     Example:

// MyQueue queue = new MyQueue();

// queue.push(1);
// queue.push(2);  
// queue.peek();  // returns 1
// queue.pop();   // returns 1
// queue.empty(); // returns false

// Notes:

// You must use only standard operations of a stack -- which means only push to top, peek/pop from top, size, and is empty operations are valid.
//     Depending on your language, stack may not be supported natively. You may simulate a stack by using a list or deque (double-ended queue), as long as you use only standard operations of a stack.
//     You may assume that all operations are valid (for example, no pop or peek operations will be called on an empty queue).



struct MyQueue {
    // use Vecs as stacks, as it supports push and pop to/of the last element
    inbox: Vec<i32>,
    outbox: Vec<i32>,
    outbox_size: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        return MyQueue {
            inbox: Vec::new(),
            outbox: Vec::new(),
            outbox_size: 0,
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.inbox.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.outbox.is_empty() {
            // grab everything from inbox
            while let Some(y) = self.inbox.pop() {
                self.outbox.push(y);
                self.outbox_size += 1;
            }
        }
        if self.outbox_size > 0 {
            self.outbox_size -= 1;
        }
        // Possibility of a panic happening if called multiple times when empty
        // so return a default value if necessary
        self.outbox.pop().unwrap_or(i32::MIN)
    }

    /** get the front element. */
    fn peek(&self) -> i32 {
        if !MyQueue::empty(self) {
            if self.outbox.is_empty() {
                return self.inbox[0];
            } else {
                return self.outbox[self.outbox_size-1];
            }
        } else {
            return i32::MIN            
        }
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.outbox.is_empty() && self.inbox.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0232() {
        // [1, 2]  []
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        // [1, 2]  []
        assert_eq!(1, q.peek());
        // [] [2, 1] 
        assert_eq!(1, q.pop());
        // [] [2]
        assert_eq!(false, q.empty());
        // [] [2]
        assert_eq!(2, q.pop());
        // [] []
        assert_eq!(true, q.empty());
        assert_eq!(i32::MIN, q.peek());
        assert_eq!(i32::MIN, q.pop());
    }
}
