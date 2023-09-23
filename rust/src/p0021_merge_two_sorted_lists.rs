/*
21. Merge Two Sorted Lists
    Easy

    Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.

    Example:

    Input: 1->2->4, 1->3->4
    Output: 1->1->2->3->4->4
 */

use crate::util::ListNode;

/*
 *   Option
 *  ---------------
 *   Box
 *  ---------------
 *   ListNode
 *  ---------------
 *
 */

// pub type ListNodeRef = Option<Box<ListNode>>;

pub struct Solution {}

impl Solution {

    pub fn sorted_merge(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head: Option<Box<ListNode>> = ListNode::new(-1);
        let mut tail: &mut Box<ListNode> = dummy_head.as_mut().unwrap();

        // While there is data left in atleast one of the lists, otherwise return list start
        // append to tail by inserting new merged node n to tail.next and advancing tail
        loop {
            match(l1.take(), l2.take()) {
                (None, None) => { // Both lists are empty, return start of new list
                    return dummy_head.unwrap().next;
                },
                (Some(n), None) | (None, Some(n)) => {
                    // Case 2
                    // Note: Don't need to adjust the l1 or l2 pointers, because if one of them is None
                    // we don't have to interleave anymore as tail.next points to the leftover sequence

                    tail.next = Some(n); // Re-wrap n from the destructure pattern match above
                },
                (Some(n1), Some(n2)) => {
                    if n1.data <= n2.data { // Since n1 is smaller, use it first
                        tail.next = ListNode::new(n1.data);
                        l1 = n1.next; // advance l1
                        l2 = Some(n2); // didn't use n2, re-wrap it
                    } else {
                        tail.next = ListNode::new(n2.data);
                        l2 = n2.next; // advance l2
                        l1 = Some(n1); // didn't use n1, re-wrap it
                    }
                }
            }

            // Advance tail
            // to avoid dropping a temporary value, we create a longer lived variable tmp using let
            // tail = &mut tail.next.as_mut().unwrap();

            let tmp = &mut tail.next;
            tail = tmp.as_mut().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0021(){
        let x = Some(Box::new(ListNode { data: 1, next: Some(Box::new(ListNode { data: 3, next: Some(Box::new(ListNode { data: 4, next: None })) })) }));

        assert_eq!(x, ListNode::to_list(&[1, 3, 4]));
        assert_eq!(ListNode::to_list(&[1, 1, 2, 3, 4, 4]), Solution::sorted_merge(ListNode::to_list(&[1, 2, 4]), ListNode::to_list(&[1, 3, 4])));
    }
}
