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

use crate::util::ListNode;

// This is the solution code using Box ListNode but the problem is how to generate
// the test condition with Box<ListNode> and not Rc<RefCell<ListSNode>>

pub struct Solution1 {}

impl Solution1 {
    pub fn has_cycle(head: &Option<Box<ListNode>>) -> bool {
        if head.is_none() { return false }

        let (mut slow, mut fast) : (&Box<ListNode>, &Box<ListNode>) =
            (head.as_ref().unwrap(), head.as_ref().unwrap());

        while fast.next != None && fast.next.as_ref().unwrap().next != None {
            fast = fast.next.as_ref().unwrap().next.as_ref().unwrap();
            slow = slow.next.as_ref().unwrap();

//            if fast == slow { return true } relax ref equality requirement to just the value
            if std::ptr::eq(fast, slow) { return true }
        }

        false
    }
}



use std::cell::RefCell;
use std::rc::Rc;

use crate::util::ListSNode;
use crate::util::ListSNodeRef;

pub struct Solution2 {}

impl Solution2 {
    pub fn has_cycle(head: &Option<Rc<RefCell<ListSNode>>>) -> bool {
        if head.is_none() { return false }

        let (mut slow, mut fast) = (head.clone().unwrap(), head.clone().unwrap());
        let (mut f, mut s): (Option<ListSNodeRef>, Option<ListSNodeRef>);

        while fast.borrow().next != None && fast.borrow().next.as_ref().unwrap().borrow().next != None {
            f = fast.borrow().next.as_ref().unwrap().borrow().next.clone();
            s = slow.borrow().next.clone();

            // Fix around lifetime borrow issues, unpack the Option into a NodeRef, vs. updating the NodeRef straight
            fast = f.unwrap();
            slow = s.unwrap();

            if Rc::ptr_eq(&fast, &slow) { return true }
        }

        false
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0141() {
        // Solution1

        // [3, 2, 0, -4] // -4 points to 2

        /*
        let mut n3 = ListNode::new(3);
        let mut n2 = ListNode::new(2);
        let mut n0 = ListNode::new(0);
        let mut n4 = ListNode::new(-4);

        // start from near end
        n4.as_mut().unwrap().next = None;
        n0.as_mut().unwrap().next = n4;
        n2.as_mut().unwrap().next = n0;

        let n2c = n2.clone();

        n3.as_mut().unwrap().next = n2;

        let mut cur = &mut n3; 

        // find last element
        while cur.as_mut().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next
        }

        cur.as_mut().unwrap().next = n2c;

        assert_eq!(true, Solution1::has_cycle(&n3));
         */

        // Solution2's

        //case0 just a straight linked list
        let n3 = ListSNode::new(3);
        let n2 = ListSNode::new(2);
        let n0 = ListSNode::new(0);
        let n4 = ListSNode::new(-4);

        // start from end
//        n4.borrow_mut().next = Some(Rc::clone(&n2));
        n0.as_ref().unwrap().borrow_mut().next = n4.clone(); 
        n2.as_ref().unwrap().borrow_mut().next = n0.clone(); 
        n3.as_ref().unwrap().borrow_mut().next = n2.clone(); 

        assert_eq!(false, Solution2::has_cycle(&n3));

        //case1
        let n3 = ListSNode::new(3);
        let n2 = ListSNode::new(2);
        let n0 = ListSNode::new(0);
        let n4 = ListSNode::new(-4);

        // start from end
        n4.as_ref().unwrap().borrow_mut().next = n2.clone(); 
        n0.as_ref().unwrap().borrow_mut().next = n4.clone(); 
        n2.as_ref().unwrap().borrow_mut().next = n0.clone(); 
        n3.as_ref().unwrap().borrow_mut().next = n2.clone(); 

        assert_eq!(true, Solution2::has_cycle(&n3));

        //case2
        let n1 = ListSNode::new(1);
        let n2 = ListSNode::new(2);

        n1.as_ref().unwrap().borrow_mut().next = n2.clone(); 
        n2.as_ref().unwrap().borrow_mut().next = n1.clone(); 

        assert_eq!(true, Solution2::has_cycle(&n1));

        //case3
        let n1 = ListSNode::new(1);

        assert_eq!(false, Solution2::has_cycle(&n1));

    }
}

/*
Without the cyclic link being inserted at -4 node:

&head = Some(
    RefCell {
        value: ListSNode {
            data: 3,
            next: Some(
                RefCell {
                    value: ListSNode {
                        data: 2,
                        next: Some(
                            RefCell {
                                value: ListSNode {
                                    data: 0,
                                    next: Some(
                                        RefCell {
                                            value: ListSNode {
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
