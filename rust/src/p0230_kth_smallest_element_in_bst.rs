/*
230. Kth Smallest Element in a BST
    Medium

    Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the
   nodes in the tree.

    Example 1:

Input: root = [3,1,4,null,2], k = 1
    Output: 1

    Example 2:

Input: root = [5,3,6,2,4,null,null,1], k = 3
    Output: 3
    */


use crate::util::TreeNodeRef;
use crate::util::TreeNode;

/*
Given tree
//
//       5
//      / \
//     2   9
//    / \
//   1  4

Algorithm is =>
1) 5 is pushed on to the stack,
2) Go left push 2,
3) Go left push 1,
4) Not able to go left, pop stack and get 1, k = 1,
5) Not able to go right as it is null
6) Pop again, get 2 and k = 2,
7) Go right, current pointing to 4, push 4 onto stack,
8) Not able to go left, pop stack and get 4 and k = 3
8) Not able to go right as its is null, pop stack and get 5 with k = 4
9) Go right, current now points to 9 and so on ...
 */


pub struct Solution {}

impl Solution {
    pub fn kth_smallest(root: &Option<TreeNodeRef>, k: i32) -> i32 {
        let not_found = 0;
        let mut count = 0;
        let mut stack: Vec<TreeNodeRef> = vec![];
        let mut last: TreeNodeRef;
        let mut left: Option<TreeNodeRef>;

        let mut current: Option<TreeNodeRef> = root.clone(); // root.as_ref().map(Rc::clone);

        // Keep going as long as there are nodes to process
        while current != None || !stack.is_empty() {

            // Traverse left subtrees (Left is where all the smaller elements are)
            while current != None {
                stack.push(current.clone().unwrap());

                // If possible keep going left, down the tree (inorder -- L N R)
                left = TreeNode::left(&current);
                current = left;
            }

            last = stack.pop().unwrap(); // if left or right is null pop stack

            // for each pop increment count, this signifies the number of lowest values seen thus far
            count += 1;

            if count == k {
                return last.borrow().data
            }

            // try last's right now as current
            current = TreeNode::right(&Some(last));

        }

        return not_found
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;
    //use std::rc::Rc;

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
