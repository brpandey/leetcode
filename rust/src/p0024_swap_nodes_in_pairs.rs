/*
24. Swap Nodes in Pairs
Medium

Given a linked list, swap every two adjacent nodes and return its head.

You may not modify the values in the list's nodes, only nodes itself may be changed.


Example:

Given 1->2->3->4, you should return the list as 2->1->4->3.
 */

use crate::util::ListNodeRef;
use crate::util::ListNode;

pub struct Solution {}

impl Solution {

    pub fn run(head: ListNodeRef) -> ListNodeRef {
        let mut dummy_head: ListNodeRef = ListNode::new(-1);
        dummy_head.as_mut().unwrap().next = head;

        let mut current: &mut ListNodeRef = &mut dummy_head;

        // Do a check if current != None and current.next != None and current.next.next != None
        // x -> a -> b -> c -> d
        while current.is_some() // x != None
            && current.as_ref().unwrap().next.is_some() // a != None
            && current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() { // b != None

                let mut first: ListNodeRef = current.as_mut().unwrap().next.take(); // a
                let second: ListNodeRef = first.as_mut().unwrap().next.take(); // b

                // current is x
                // x now points to the sequence: x -> b -> a -> c -> d
                current.as_mut().unwrap().next = Solution::swap(first, second);

                // current is now a or current.next.next (now setup to flip c -> d on the next loop iteration)
                current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
            }

        return dummy_head.unwrap().next.take();
    }

    // a -> b -> c, where first is a and second is b, changes to b -> a -> c

    //              a                       b
    pub fn swap(mut first: ListNodeRef, mut second: ListNodeRef) -> ListNodeRef {
        // a -> c
        first.as_mut().unwrap().next = second.as_mut().unwrap().next.take();
        // b -> a -> c
        second.as_mut().unwrap().next = first;
        return second;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0024(){
        assert_eq!(ListNode::to_list(&[2, 1, 4, 3]),
                   Solution::run(ListNode::to_list(&[1,2,3,4])));
    }
}
