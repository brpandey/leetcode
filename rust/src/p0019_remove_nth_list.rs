use crate::util::ListNode;
use crate::util::NodeRef;

pub struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: &Option<NodeRef<u32>>, n: u32) -> Option<NodeRef<u32>> {
        let dummy: NodeRef<u32> = ListNode::new(u32::MIN);
        dummy.borrow_mut().next = ListNode::clone(head);

        let mut first: Option<NodeRef<u32>> = ListNode::clone(head);
        let mut second: Option<NodeRef<u32>> = ListNode::clone(head);

        // Advance first so that first and second are n nodes apart
        for _ in 0..=n {
            first = ListNode::next(&first);
        }

        // Move first to the list end keeping that gap of n nodes
        while first.is_some() {
            first = ListNode::next(&first);
            second = ListNode::next(&second);
        }

        let remove = ListNode::next(&second);
        let remove_next = ListNode::next(&remove);

        // (old comment)
        // Using interior mutability make immutable reference mutable (using runtime checks instead of compile time)
        // as_ref takes an Option<T> and turns it into a Option<&T>, so we can borrow the inside
        // unwrap takes the value inside the Option<&T> hence giving us a &T or &ListNode<u32>

        // (new comment)
        // link second to node after remove which is remove_next, in effect dropping the remove node or the nth node from the end
        // nothing is pointing to remove after this function, so it is dropped
        second.as_ref().unwrap().borrow_mut().next = remove_next;

        let result = dummy.borrow_mut().next.take();
        result

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0019(){
        assert_eq!(ListNode::from_list(&[1, 2, 3, 4, 5]), Solution::remove_nth_from_end(&ListNode::from_list(&[1, 2, 3, 4, 5]), 0));
        assert_eq!(ListNode::from_list(&[1, 2, 3, 4]), Solution::remove_nth_from_end(&ListNode::from_list(&[1, 2, 3, 4, 5]), 1));
        assert_eq!(ListNode::from_list(&[1, 2, 3, 5]), Solution::remove_nth_from_end(&ListNode::from_list(&[1, 2, 3, 4, 5]), 2));
        assert_eq!(ListNode::from_list(&[1, 2, 4, 5]), Solution::remove_nth_from_end(&ListNode::from_list(&[1, 2, 3, 4, 5]), 3));
        assert_eq!(ListNode::from_list(&[1, 3, 4, 5]), Solution::remove_nth_from_end(&ListNode::from_list(&[1, 2, 3, 4, 5]), 4));
    }
}

/*

array [1, 2, 3, 4, 5] is turned into this piece of ascii art-->

Some(
    RefCell {
        value: ListNode {
            data: 1,
            next: Some(
                RefCell {
                    value: ListNode {
                        data: 2,
                        next: Some(
                            RefCell {
                                value: ListNode {
                                    data: 3,
                                    next: Some(
                                        RefCell {
                                            value: ListNode {
                                                data: 4,
                                                next: Some(
                                                    RefCell {
                                                        value: ListNode {
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
