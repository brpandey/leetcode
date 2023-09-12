/*
206. Reverse Linked List
Easy

Given the head of a singly linked list, reverse the list, and return the reversed list.

*/
// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
}

impl ListNode {
   #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None
        }

        let mut former_forward_neighbor;
        let mut current = head;
        let mut reversed_dummy_head = ListNode::new(-1);

        // 1-> 2 -> None

        // while loop first
        // node -> 1 -> 2 -> None
        // temp -> 2
        // n.next -> None
        // reversed.next -> 1 -> None
        // node -> 2 -> None

        // while loop second
        // node -> 2 -> None
        // temp -> None
        // n.next -> 1-> None
        // reversed.next -> 2 -> 1 -> None
        // node -> None

        while let Some(mut c) = current {
            former_forward_neighbor = c.next.take(); // move node's next neighbor aside for a second
//            println!("c is {:?}, former_forward_neighbor is {:?}", &c, &former_forward_neighbor);
            c.next = reversed_dummy_head.next; // point node next to point back to the last node handled
            reversed_dummy_head.next = Some(c); // update reversed next to point to current node
            current = former_forward_neighbor; // update node to its former neighbor
        }

        // This points to the last node in the old sequence
        // since reversed isn't part of the original list it is discarded,
        // leaving the Option<Box<ListNode>> it points to
        reversed_dummy_head.next
    }

    pub fn from_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        if list.is_empty() {
            return None
        }

        let mut dummy_head = ListNode::new(-1);
        let mut node;

        // reverse list as its easier to
        for l in list.iter().rev() {
            node = ListNode::new(*l); // create new node w/o box or option
            node.next = dummy_head.next; // Node next points to previous head.next
            dummy_head.next = Some(Box::new(node)); // Update head.next to current node
        }

        dummy_head.next
    }

    // keep the original list intact
    pub fn to_list(head: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![];
        let mut node = head;

        // walk the linked list of nodes, pushing the data value onto vec
        while let Some(n) = node.as_ref() {
            output.push(n.val);
            node = &n.next;
        }

        output
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0206() {
        let empty_vec: Vec<i32> = vec![];
        // test1
        let original = Solution::from_list(vec![1,2,3,4,5]);
//        dbg!(&original);
        let reversed = Solution::reverse_list(original);
//        dbg!(&reversed);
        assert_eq!(vec![5,4,3,2,1], Solution::to_list(&reversed));

        // test2
        assert_eq!(vec![2,1], Solution::to_list(&Solution::reverse_list(Solution::from_list(vec![1,2]))));
        assert_eq!(empty_vec, Solution::to_list(&Solution::reverse_list(Solution::from_list(vec![]))));
    }


//  Test 1
//  Original
/*
    &original = Some(
        ListNode {
            val: 1,
            next: Some(
                ListNode {
                    val: 2,
                    next: Some(
                        ListNode {
                            val: 3,
                            next: Some(
                                ListNode {
                                    val: 4,
                                    next: Some(
                                        ListNode {
                                            val: 5,
                                            next: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    )

    &reversed = Some(
        ListNode {
            val: 5,
            next: Some(
                ListNode {
                    val: 4,
                    next: Some(
                        ListNode {
                            val: 3,
                            next: Some(
                                ListNode {
                                    val: 2,
                                    next: Some(
                                        ListNode {
                                            val: 1,
                                            next: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    )
     */

}

