// Solution

use crate::util::ListNodeRef;

// Solution1
use crate::util::ListSNode;
use crate::util::ListSNodeRef;


/* List Node solution using Option<Box<ListNode>*/
pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(mut head: ListNodeRef, n: u32) -> ListNodeRef {
        if n == 0 { return head } // nothing to delete

        let mut count: u32 = 0;
        let mut current: &ListNodeRef = &head;

        // Find the node count in linked list
        while current.is_some() {
            current = &current.as_ref().unwrap().next;
            count += 1;
        }

        let stop = count - n;
        let mut current = &mut head;

        // Stop node at nth position
        for _ in 0..stop-1 {
            current = &mut current.as_mut().unwrap().next;
        }

        //            nth node (delete node)
        // current -> delete -> keep (before)
        // current -> keep (after)

        let mut delete_segment = current.as_mut().unwrap().next.take();
        let mut keep_segment = None;

        // otherwise if n is 1, delete_segment is null, so can't unwrap it
        if n > 1 {
            keep_segment = delete_segment.as_mut().unwrap().next.take();
        } // or could use: keep_segment = delete_segment.as_mut().and_then(|d| { d.next.take() }); 

        // stitch together
        current.as_mut().unwrap().next = keep_segment;

        head
    }

}

/* List Node solution using Reference-counted ListNodes (my preference) */

pub struct Solution1 {}

impl Solution1 {
    // head = &Option<Rc<RefCell<Node<T>>>>
    pub fn remove_nth_from_end(head: &ListSNodeRef, n: u32) -> ListSNodeRef {
        let dummy: ListSNodeRef = ListSNode::new(-1);
        dummy.as_ref().unwrap().borrow_mut().next = ListSNode::clone(head);

        let mut first: ListSNodeRef = ListSNode::clone(head);
        let mut second: ListSNodeRef = ListSNode::clone(head);

        // Advance first so that first and second are n nodes apart
        for _ in 0..=n {
            first = ListSNode::next(&first);
        }

        // Move first to the list end keeping that gap of n nodes
        while first.is_some() {
            first = ListSNode::next(&first);
            second = ListSNode::next(&second);
        }

        let remove = ListSNode::next(&second);
        let remove_next = ListSNode::next(&remove);

        // Option (as_ref, unwrap), RefCell (borrow_mut(), interior mutability), .next (ListSNode
        // field)
        second.as_ref().unwrap().borrow_mut().next = remove_next;

        let result = dummy.as_ref().unwrap().borrow_mut().next.take();
        result

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::ListNode;

    #[test]
    fn test_0019(){
        //Solution1
        assert_eq!(ListSNode::from_list(&[1, 2, 3, 4, 5]), Solution1::remove_nth_from_end(&ListSNode::from_list(&[1, 2, 3, 4, 5]), 0));
        assert_eq!(ListSNode::from_list(&[1, 2, 3, 4]), Solution1::remove_nth_from_end(&ListSNode::from_list(&[1, 2, 3, 4, 5]), 1));
        assert_eq!(ListSNode::from_list(&[1, 2, 3, 5]), Solution1::remove_nth_from_end(&ListSNode::from_list(&[1, 2, 3, 4, 5]), 2));
        assert_eq!(ListSNode::from_list(&[1, 2, 4, 5]), Solution1::remove_nth_from_end(&ListSNode::from_list(&[1, 2, 3, 4, 5]), 3));
        assert_eq!(ListSNode::from_list(&[1, 3, 4, 5]), Solution1::remove_nth_from_end(&ListSNode::from_list(&[1, 2, 3, 4, 5]), 4));

        //Solution
        assert_eq!(ListNode::to_list(&[1, 2, 3, 4, 5]), Solution::remove_nth_from_end(ListNode::to_list(&[1, 2, 3, 4, 5]), 0));
        assert_eq!(ListNode::to_list(&[1, 2, 3, 4]), Solution::remove_nth_from_end(ListNode::to_list(&[1, 2, 3, 4, 5]), 1));
        assert_eq!(ListNode::to_list(&[1, 2, 3, 5]), Solution::remove_nth_from_end(ListNode::to_list(&[1, 2, 3, 4, 5]), 2));
        assert_eq!(ListNode::to_list(&[1, 2, 4, 5]), Solution::remove_nth_from_end(ListNode::to_list(&[1, 2, 3, 4, 5]), 3));
        assert_eq!(ListNode::to_list(&[1, 3, 4, 5]), Solution::remove_nth_from_end(ListNode::to_list(&[1, 2, 3, 4, 5]), 4));

    }
}

/*

array [1, 2, 3, 4, 5] is turned into this piece of ascii art-->

Some(
    RefCell {
        value: ListSNode {
            data: 1,
            next: Some(
                RefCell {
                    value: ListSNode {
                        data: 2,
                        next: Some(
                            RefCell {
                                value: ListSNode {
                                    data: 3,
                                    next: Some(
                                        RefCell {
                                            value: ListSNode {
                                                data: 4,
                                                next: Some(
                                                    RefCell {
                                                        value: ListSNode {
                                                            data: 5,
                                                            next: None,
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
                                },
                            },
                        ),
                    },
                },
            ),
        },
    },
)

 */
