/*
141. Linked List Cycle
Easy

Given head, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.

Return true if there is a cycle in the linked list. Otherwise, return false.

Example 1:

Input: head = [3,2,0,-4], pos = 1
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).

Example 2:

Input: head = [1,2], pos = 0
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.

Example 3:

Input: head = [1], pos = -1
Output: false
Explanation: There is no cycle in the linked list.


Constraints:

The number of the nodes in the list is in the range [0, 104].
-105 <= Node.val <= 105
pos is -1 or a valid index in the linked-list.


*/

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<ListNode<T>>>;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode<T> {
    pub data: T,
    pub next: Option<NodeRef<T>>
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> NodeRef<T> {
        Rc::new(
            RefCell::new(
                ListNode {
                    next: None,
                    data
                }
            )
        )
    }
}


/*
    3 -> 2 -> 0 -> -4 (-4 is connected to 2)
    - is fast
    + is slow

    3 -> 2 -> 0 -> -4
    -
    +

    3 -> 2 -> 0 -> -4
              -
         +

    3 -> 2 -> 0 -> -4
         -
              +

    3 -> 2 -> 0 -> -4
                   -
                   +
                  MEET!
*/


pub struct Solution {}

impl Solution {
    pub fn has_cycle(head: &Option<NodeRef<i32>>) -> bool {
        if *head == None { return false }

        let mut slow: NodeRef<i32> = Rc::clone(head.as_ref().unwrap());
        let mut fast: NodeRef<i32> = Rc::clone(head.as_ref().unwrap());

        let mut f: Option<NodeRef<i32>>;
        let mut s: Option<NodeRef<i32>>;

        while fast.borrow().next != None && fast.borrow().next.as_ref().unwrap().borrow().next != None {

            f = fast.borrow().next.as_ref().unwrap().borrow().next.as_ref().map(Rc::clone);
            s = slow.borrow().next.as_ref().map(Rc::clone);

            // Fix around lifetime borrow issues, unpack the Option into a NodeRef, vs. updating the NodeRef straight
            fast = f.unwrap();
            slow = s.unwrap();

            if Rc::ptr_eq(&fast, &slow) {
                // dbg!(&fast.borrow().data); should be -4
                return true
            }

        }

        false
  }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0141() {
        //case0 just a straight linked list
        let n3 = ListNode::new(3);
        let n2 = ListNode::new(2);
        let n0 = ListNode::new(0);
        let n4 = ListNode::new(-4);

        // start from end
//        n4.borrow_mut().next = Some(Rc::clone(&n2));
        n0.borrow_mut().next = Some(Rc::clone(&n4));
        n2.borrow_mut().next = Some(Rc::clone(&n0));
        n3.borrow_mut().next = Some(Rc::clone(&n2));

        let head = Some(n3); 

        assert_eq!(false, Solution::has_cycle(&head));



        //case1
        let n3 = ListNode::new(3);
        let n2 = ListNode::new(2);
        let n0 = ListNode::new(0);
        let n4 = ListNode::new(-4);

        // start from end
        n4.borrow_mut().next = Some(Rc::clone(&n2));
        n0.borrow_mut().next = Some(Rc::clone(&n4));
        n2.borrow_mut().next = Some(Rc::clone(&n0));
        n3.borrow_mut().next = Some(Rc::clone(&n2));

        let head = Some(n3); 

        assert_eq!(true, Solution::has_cycle(&head));


        //case2
        let n1 = ListNode::new(1);
        let n2 = ListNode::new(2);

        n1.borrow_mut().next = Some(Rc::clone(&n2));
        n2.borrow_mut().next = Some(Rc::clone(&n1));

        let head = Some(n1);

        assert_eq!(true, Solution::has_cycle(&head));

        //case3
        let n1 = ListNode::new(1);
        let head = Some(n1);

        assert_eq!(false, Solution::has_cycle(&head));

    }
}

/*
Without the cyclic link being inserted at -4 node:

&head = Some(
    RefCell {
        value: ListNode {
            data: 3,
            next: Some(
                RefCell {
                    value: ListNode {
                        data: 2,
                        next: Some(
                            RefCell {
                                value: ListNode {
                                    data: 0,
                                    next: Some(
                                        RefCell {
                                            value: ListNode {
                                                data: -4,
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
)
*/
