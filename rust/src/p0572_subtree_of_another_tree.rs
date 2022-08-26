/*
572. Subtree of Another Tree
Easy

Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.

A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.



Example 1:

Input: root = [3,4,5,1,2], subRoot = [4,1,2]
Output: true

Example 2:

Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
Output: false



Constraints:

The number of nodes in the root tree is in the range [1, 2000].
The number of nodes in the subRoot tree is in the range [1, 1000].
-104 <= root.val <= 104
-104 <= subRoot.val <= 104


*/

use crate::util::TreeNode;
use crate::util::TreeNodeRef as TreeNodeRef;
use std::rc::Rc;

pub struct Solution {}


impl Solution {
    pub fn is_subtree(root: Option<TreeNodeRef>, sub_root: Option<TreeNodeRef>) -> bool {
        if sub_root == None { return true }
        if root == None { return false }

        // create the extra ref ptrs to the root and subroot as we need
        let r1 = root.as_ref().map(Rc::clone);
        let r2 = root.as_ref().map(Rc::clone);

        let s1 = sub_root.as_ref().map(Rc::clone);
        let s2 = sub_root.as_ref().map(Rc::clone);
        let s3 = sub_root.as_ref().map(Rc::clone);

        if Self::is_same(r1, s1) { return true }

        // adjust the tree by taking the sub_root and matching against its left or right subtrees
        Self::is_subtree(TreeNode::left(&r2), s2) || Self::is_subtree(TreeNode::right(&r2), s3)
    }

    pub fn is_same(node1: Option<TreeNodeRef>, node2: Option<TreeNodeRef>) -> bool {
        if node1 == None && node2 == None {
            return true
        }

        if node1 == None || node2 == None {
            return false
        }

        // If the current node value matches, check the that the l & r subtrees both match, recursively
        if TreeNode::value(&node1) == TreeNode::value(&node2) {
            return Self::is_same(TreeNode::left(&node1), TreeNode::left(&node2)) &&
                Self::is_same(TreeNode::right(&node1), TreeNode::right(&node2))
        }

        // if no match up to this point the nodes/trees aren't the same
        false
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0572() {
        let root = TreeNode::from_list(vec![3,4,5,1,2]);
        let sub_root = TreeNode::from_list(vec![4,1,2]);
        assert_eq!(true, Solution::is_subtree(root, sub_root));

        let root = TreeNode::from_list(vec![3,4,5,1,2,i32::MIN,i32::MIN,i32::MIN,i32::MIN,0]);
        let sub_root = TreeNode::from_list(vec![4,1,2]);
        assert_eq!(false, Solution::is_subtree(root, sub_root));

    }
}

