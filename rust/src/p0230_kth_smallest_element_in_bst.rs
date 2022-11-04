/*
230. Kth Smallest Element in a BST
    Medium

    Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.

    Example 1:

Input: root = [3,1,4,null,2], k = 1
    Output: 1

    Example 2:

Input: root = [5,3,6,2,4,null,null,1], k = 3
    Output: 3
    */

use std::rc::Rc;
use std::cell::RefCell;
pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }

 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> TreeNodeRef {
       Rc::new(
           RefCell::new(
               TreeNode {
                   val,
                   left: None,
                   right: None
               }
           )
       )
   }
 }

pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: &Option<TreeNodeRef>, k: i32) -> i32 {
        let not_found = 0;
        let mut stack: Vec<TreeNodeRef> = vec![];
        let mut cnt = 0;
        let mut rc_node: TreeNodeRef;
        let mut temp: Option<TreeNodeRef>;
        let mut current: Option<TreeNodeRef> = root.as_ref().map(Rc::clone);
//        stack.push(Rc::clone(root.as_ref().unwrap())); move it into second while loop

        // Keep going as long as there are things to process
        while current != None || !stack.is_empty() {

            // Traverse left subtrees
            while current != None {

                stack.push(Rc::clone(&current.as_ref().unwrap()));

                // if possible keep going left down the tree (inorder -- L N R)
                //
                //       5
                //      / \
                //     2   9
                //    / \
                //   1  4

                // Left is where all the smaller elements are
                temp = current.as_ref().unwrap().borrow().left.as_ref().map(Rc::clone);
//                if temp != None { stack.push(Rc::clone(&temp.as_ref().unwrap())) } // move to top of block, add current != None on top while loop
                current = temp;
            }

            // For example, say we've pushed 5 on to the stack, gone left pushed 2 as well, gone left again, pushed 1,
            // we're not able to go left anymore, we pop and rc_node is node 1 and k = 1,
            // we try to go right, it is null as well,
            // pop again, rc_node is now 2 and k = 2,
            // we go right with current pointing to 4, push 4 onto stack, can't go left anymore, so we pop 4 and k = 3
            // we go right to no luck, we pop again and rc_node is node 5 with k = 4, we go right, current now points to 9 and so on ...

            rc_node = stack.pop().unwrap();

            // everytime we pop, increment count, this signifies the number of lowest values seen thus far
            cnt += 1;

            if cnt == k {
                return rc_node.borrow().val
            }

            // update current with a cloned RC to right node
            current = rc_node.borrow().right.as_ref().map(Rc::clone);

        }

        return not_found
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0230() {

        // Test 1
        //Input: root = [3,1,4,null,2], k = 1

        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);

        node1.borrow_mut().right = Some(node2);
        node3.borrow_mut().left = Some(node1);
        node3.borrow_mut().right = Some(node4);

        let head = Some(node3);

        assert_eq!(1, Solution::kth_smallest(&head,1));

        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);
        let node6 = TreeNode::new(6);


        node2.borrow_mut().left = Some(node1);
        node3.borrow_mut().left = Some(node2);
        node3.borrow_mut().right = Some(node4);
        node5.borrow_mut().left = Some(node3);
        node5.borrow_mut().right = Some(node6);

        let head = Some(node5);

        assert_eq!(3, Solution::kth_smallest(&head,3));

    }
}
