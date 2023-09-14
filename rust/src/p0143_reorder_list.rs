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

use crate::util::ListNodeRef;
use crate::util::ListNode;

use crate::util::ListSNodeRef;
use crate::util::ListSNode;

/* Solution uses no shared nodes, just Box<ListNode> */

pub struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut ListNodeRef) {
        let second = Self::split_by_mid(head);

        let mut reversed = Self::reverse(second); // Reverse second half to setup for stitching

        let merged = Self::merge(head, &mut reversed);

        *head = merged
    }

    pub fn split_by_mid(head: &mut ListNodeRef) -> ListNodeRef {
        let mut current: &ListNodeRef = head;
        let mut current_unwrapped;
        let mut count: usize = 0;

        // 1 -> 2 -> 3 -> 4, midpoint is ( 4 / 2 ) + ( 4 % 2 ) = 2nd element
        while current.is_some() {
            current_unwrapped = current.as_ref().unwrap();
            current = &current_unwrapped.next;
            count += 1;
        }

        let mid_point = count / 2 + count % 2;

        let mut node: &mut ListNodeRef = head;
        let mut n: &mut Box<ListNode>;

        for _i in 0..mid_point {
            n = node.as_mut().unwrap();
            node = &mut n.next
        }

        let second = node.take();
        second
    }

    pub fn merge(list1: &mut ListNodeRef, list2: &mut ListNodeRef) -> ListNodeRef{
        let mut l1 = list1;
        let mut l2 = list2;

        // Now stitch the nodes from the first and second lists into the output list

        // New list, note: the original head is always the start of the new list (so don't need to return anything)
        let mut out = ListNode::new(-1);
        let mut merged = &mut out;

        let mut l1_next;
        let mut l2_next;
        let mut n1: ListNodeRef;
        let mut n2: ListNodeRef;

        // Add nodes on to the new list (out) and increment the ptrs for out, l1 and l2 as needed
        while l1.is_some() || l2.is_some() {

            if l1.is_some() {
                n1 = l1.take();
                l1_next = n1.as_mut().unwrap().next.take();

                merged.as_mut().unwrap().next = n1; // ListNode::clone(&l1);
                merged = &mut merged.as_mut().unwrap().next;
                l1 = &mut l1_next;
            }

            if l2.is_some() {
                n2 = l2.take();
                l2_next = n2.as_mut().unwrap().next.take();

                merged.as_mut().unwrap().next = n2; // ListNode::clone(&l2);
                merged = &mut merged.as_mut().unwrap().next;
                l2 = &mut l2_next;
            }
        }
        out.as_mut().unwrap().next.take()
    }

    pub fn reverse(mut current: ListNodeRef) -> ListNodeRef {
        let mut reversed_dummy_head = ListNode::new(-1);
        let mut forward_next;

        // 1 -> 2 -> 3 -> 4 -> NULL becomes
        // NULL <- 1 and 2 -> 3 -> 4 -> NULL which becomes
        // NULL <- 1 <- 2 and 3 -> 4 -> NULL and so on...

        while current.is_some() {
            forward_next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = reversed_dummy_head.as_mut().unwrap().next.take();
            reversed_dummy_head.as_mut().unwrap().next = current.take();
            current = forward_next;
        }

        let result = reversed_dummy_head.as_mut().unwrap().next.take();
        result
    }
}

/* Solution 1 uses RefCell'ed (Shared) ListNodes */

pub struct Solution1 {}

impl Solution1 {
    pub fn reorder_list(head: &ListSNodeRef) {
        let first: ListSNodeRef = ListSNode::clone(head);
        let mut second = Self::find_mid(head);
        second = Self::reverse(second); // Reverse second half to setup for stitching
        Self::merge(first, second);
    }

    pub fn find_mid(head: &ListSNodeRef) -> ListSNodeRef {
        let mut first: ListSNodeRef = ListSNode::clone(head);
        let mut second: ListSNodeRef = ListSNode::clone(head);

        // first while iteration
        // s    second while iteration
        // f    s    f
        // 1 -> 2 -> 3 -> 4

        // Move fast and slow pointer, when fast is null take second.next as the middle of the list
        // First is used as fast, second is used as slow
        while ListSNode::next(&first).is_some() && ListSNode::next(&ListSNode::next(&first)).is_some() {
            first = ListSNode::next(&first);
            first = ListSNode::next(&first);
            second = ListSNode::next(&second);
        }

        // Set end to first half of list
        let first_end = ListSNode::clone(&second);

        // Advance second to the start of the second list
        second = ListSNode::next(&second);

        ListSNode::set_next(&first_end, None); // first_end.as_ref().unwrap().borrow_mut().next = None;

        return second
    }


    pub fn merge(mut l1: ListSNodeRef, mut l2: ListSNodeRef) {
        // Now stitch the nodes from the first and second lists into the output list

        // New list, note: the original head is always the start of the new list (so don't need to return anything)
        let mut nl = ListSNode::new(i32::MIN);

        // Add nodes on to the new list (nl) and increment the ptrs for nl and l1 and l2 as needed
        while l1 != None || l2 != None {
            if l1.is_some() {
                nl.as_ref().unwrap().borrow_mut().next = ListSNode::clone(&l1);
                nl = ListSNode::next(&nl);
                l1 = ListSNode::next(&l1);
            }

            if l2.is_some() {
                nl.as_ref().unwrap().borrow_mut().next = ListSNode::clone(&l2);
                nl = ListSNode::next(&nl);
                l2 = ListSNode::next(&l2);
            }
        }
    }

    pub fn reverse(mut current: ListSNodeRef) -> ListSNodeRef {
        let reversed_dummy_head = ListSNode::new(-1);
        let mut forward_next;

        // 1 -> 2 -> 3 -> 4 -> NULL becomes
        // NULL <- 1 and 2 -> 3 -> 4 -> NULL which becomes
        // NULL <- 1 <- 2 and 3 -> 4 -> NULL and so on...

        while current != None {
            forward_next = ListSNode::next(&current);
            current.as_ref().unwrap().borrow_mut().next = ListSNode::next(&reversed_dummy_head);
            reversed_dummy_head.as_ref().unwrap().borrow_mut().next = ListSNode::clone(&current);
            current = forward_next;
        }

        let result = reversed_dummy_head.as_ref().unwrap().borrow_mut().next.take();
        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0143() {
        /*
        let head = ListSNode::from_list(&vec![1,2,3,4]);
        Solution1::reorder_list(&head);
        assert_eq!(ListSNode::from_list(&vec![1,4,2,3]), head);

        let head = ListSNode::from_list(&vec![1,2,3,4,5]);
        Solution1::reorder_list(&head);
        assert_eq!(ListSNode::from_list(&vec![1,5,2,4,3]), head);
         */

        // Solution
        let mut head = ListNode::to_list(&vec![1,2,3,4]);
        Solution::reorder_list(&mut head);
        assert_eq!(ListNode::to_list(&vec![1,4,2,3]), head);

        let mut head = ListNode::to_list(&vec![1,2,3,4,5]);
        Solution::reorder_list(&mut head);
        assert_eq!(ListNode::to_list(&vec![1,5,2,4,3]), head);

    }
}
