/*
 * 104. Maximum Depth of Binary Tree
Easy

Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.


Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: 3

Example 2:

Input: root = [1,null,2]
Output: 2


Constraints:

    The number of nodes in the tree is in the range [0, 104].
    -100 <= Node.val <= 100

 */

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<TreeNodeRef>) -> i32 {
        if root.is_none() { return 0 }

        let l = TreeNode::left(&root);
        let r = TreeNode::right(&root);

        return 1 + std::cmp::max(Solution::max_depth(l), Solution::max_depth(r));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::rc::Rc;
    
    #[test]
    pub fn test_0104() {
        let node3 = TreeNode::new(3);
        let node9 = TreeNode::new(9);
        let node20 = TreeNode::new(20);
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);

        node3.borrow_mut().left = Some(node9);
        node3.borrow_mut().right = Some(Rc::clone(&node20));

        node20.borrow_mut().left = Some(node15);
        node20.borrow_mut().right = Some(node7);

        let head = Some(node3);

        assert_eq!(3, Solution::max_depth(head));

        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);

        node1.borrow_mut().right = Some(node2);

        let head = Some(node1);

        assert_eq!(2, Solution::max_depth(head));
    }
}

