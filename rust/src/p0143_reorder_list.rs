/*
 *
 * 143. Reorder List
Medium

You are given the head of a singly linked-list. The list can be represented as:

L0 → L1 → … → Ln - 1 → Ln

Reorder the list to be on the following form:

L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …

You may not modify the values in the list's nodes. Only nodes themselves may be changed.

 

Example 1:

Input: head = [1,2,3,4]
Output: [1,4,2,3]

Example 2:

Input: head = [1,2,3,4,5]
Output: [1,5,2,4,3]

 

Constraints:

    The number of nodes in the list is in the range [1, 5 * 104].
    1 <= Node.val <= 1000


 */

/*
Strategy:

1 -> 2 -> 3 -> 4 -> 5 can be split into two halves: l1: 1 -> 2 -> 3   l2: 4 -> 5

If we reverse l2 this becomes:                      l1: 1 -> 2 -> 3   l2: 5 -> 4

Now if we stitch the two together taking one element from l1 and l2 as available, create a new list:

1 -> 5 -> 2 -> 4 -> 3

This is the fastest O(n)

The other approach which is much slower of O(n^2)

is reversing the list as we iterate through it, which is n -1 + n -2 + n - 3 ... n^2

1 -> 5 -> 4 -> 3 -> 2  then 1 -> 5 -> 2 -> 3 -> 4 then 1 -> 5 -> 2 -> 4 -> 3 and then 1 -> 5 -> 2 -> 4 -> 3
     -                                -                               -                                   -
*/

use crate::util::NodeRef;
use crate::util::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &Option<NodeRef<i32>>) {
        let mut first: Option<NodeRef<i32>> = ListNode::clone(head);
        let mut second: Option<NodeRef<i32>> = ListNode::clone(head);

        // first while iteration
        // s    second while iteration
        // f    s    f
        // 1 -> 2 -> 3 -> 4

        // Move fast and slow pointer, when fast is null take second.next as the middle of the list
        // First is used as fast, second is used as slow
        while ListNode::next(&first).is_some() && ListNode::next(&ListNode::next(&first)).is_some() {
            first = ListNode::next(&first);
            first = ListNode::next(&first);
            second = ListNode::next(&second);
        }

        // First now points to the first half of the list, with second the second half
        first = ListNode::clone(head);
        let first_end = ListNode::clone(&second);

        // 1) Advance second to the start of the second list
        // 2) Set end to first half of list
        // 3) Reverse second half to setup for stitching
        second = ListNode::next(&second); 
        first_end.as_ref().unwrap().borrow_mut().next = None;
        second = Self::reverse(second);

        // Now stitch the nodes from the first and second lists into the output list
        let mut l1 = first;
        let mut l2 = second;

        // New list, note: the original head is always the start of the new list (so don't need to return anything)
        let mut nl = Some(ListNode::new(i32::MIN));

        // Add nodes on to the new list (nl) and increment the ptrs for nl and l1 and l2 as needed
        while l1 != None || l2 != None {
            if l1.is_some() {
                nl.as_ref().unwrap().borrow_mut().next = ListNode::clone(&l1);
                nl = ListNode::next(&nl);
                l1 = ListNode::next(&l1);
            }

            if l2.is_some() {
                nl.as_ref().unwrap().borrow_mut().next = ListNode::clone(&l2);
                nl = ListNode::next(&nl);
                l2 = ListNode::next(&l2);
            }
        }
    }

    pub fn reverse(mut head: Option<NodeRef<i32>>) -> Option<NodeRef<i32>> {
        let tail = Some(ListNode::new(i32::MIN));
        let mut temp;

        // 1 -> 2 -> 3 -> 4 -> NULL becomes
        // NULL <- 1 and 2 -> 3 -> 4 -> NULL which becomes
        // NULL <- 1 <- 2 and 3 -> 4 -> NULL and so on...
        while head != None {
            temp = ListNode::next(&head);
            head.as_ref().unwrap().borrow_mut().next = ListNode::next(&tail);
            tail.as_ref().unwrap().borrow_mut().next = ListNode::clone(&head);
            head = temp;
        }

        let result = tail.as_ref().unwrap().borrow_mut().next.take();
        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0143() {
        let head = ListNode::from_list(&vec![1,2,3,4]);
        Solution::reorder_list(&head);
        assert_eq!(ListNode::from_list(&vec![1,4,2,3]), head);

        let head = ListNode::from_list(&vec![1,2,3,4,5]);
        Solution::reorder_list(&head);
        assert_eq!(ListNode::from_list(&vec![1,5,2,4,3]), head);
    }
}
