/*
21. Merge Two Sorted Lists
    Easy

    Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.

    Example:

Input: 1->2->4, 1->3->4
    Output: 1->1->2->3->4->4
 */

pub type NodeLink<T> = Option<Box<ListNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub data: T,
    pub next: NodeLink<T>
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> NodeLink<T> {
        Some(Box::new(ListNode {
            next: None,
            data
        }))
    }
}

/*
 *   Option
 *  ---------------
 *   Box
 *  ---------------
 *   ListNode
 *  ---------------
 *
 */

pub struct Solution {}

impl Solution {
    // pub type NodeLink<T> = Option<Box<ListNode<T>>>;
    pub fn sorted_merge(mut l1: NodeLink<u32>, mut l2: NodeLink<u32>) -> NodeLink<u32> {
        // This needs to be mut since as we need a &mut pointing to it
        let mut dummy_head: NodeLink<u32> = ListNode::new(0);

        // This needs to be &mut Option<Box<ListNode.. 
        let mut tail: &mut NodeLink<u32> = &mut dummy_head;

        // This is somewhat similiar to problem 2, add two numbers
        loop {
            // Retrieve tail and make it mutable
            // Append to tail by modifiying the last node that tail points to
            // and changing that node's next pointer to the n
            let mut last = tail.as_mut().unwrap();

            match(l1.take(), l2.take()) {
                // Both lists are empty now
                (None, None) => {
                    return dummy_head.unwrap().next;
                },
                // List 1 still has an element, list 2 is empty
                // OR list 1 is empty, list 2 still has an element
                (Some(n), None) | (None, Some(n)) => {
                    // Case 2
                    // Re-wrap n from the destructure pattern match above
                    // Note: we don't need to adjust the l1 or l2 pointers
                    last.next = Some(n);
                },
                (Some(n1), Some(n2)) => {
                    if n1.data <= n2.data {
                        last.next = ListNode::new(n1.data);
                        l1 = n1.next;
                        l2 = Some(n2);
                    } else {
                        last.next = ListNode::new(n2.data);
                        l2 = n2.next;
                        l1 = Some(n1);
                    }
                }
            }
            // Update tail
            // For Case 2, note if there are elements after l1 or l2 (whichever is nonempty),
            // this is handled with tail now pointing to the leftover sequence
            tail = &mut last.next;
        }
    }


    pub fn to_list(a: &[u32]) -> NodeLink<u32> {
        let mut head: NodeLink<u32> = None;

        // Reverse the array list so 4 then points to 5 etc..
        for &v in a.iter().rev() {
            let mut n = (ListNode::new(v)).unwrap();
            n.next = head;
            head = Some(n);
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0021(){
        let x = Some(Box::new(ListNode { data: 1, next: Some(Box::new(ListNode { data: 3, next: Some(Box::new(ListNode { data: 4, next: None })) })) }));

        assert_eq!(x, Solution::to_list(&[1, 3, 4]));
        assert_eq!(Solution::to_list(&[1, 1, 2, 3, 4, 4]), Solution::sorted_merge(Solution::to_list(&[1, 2, 4]), Solution::to_list(&[1, 3, 4])));
    }
}
