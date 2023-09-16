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

pub struct Solution {}

impl Solution {
    // check if subroot and root are same, otherwise traverse the l and r subtrees of root
    // checking if they match the subroot
    pub fn is_subtree(root: Option<TreeNodeRef>, sub_root: Option<TreeNodeRef>) -> bool {
        if sub_root == None { return true } // base case 1: the root's empty leaves match an empty subroot 
        if root == None { return false } // base case 2: if root is empty, nothing can be a smaller subtree of it 

        // create the extra ref ptrs to the subroot
        let (sr1, sr2, sr3) = (sub_root.clone(), sub_root.clone(), sub_root.clone());

        if Self::is_same(root.clone(), sr1) { return true }

        // adjust view of root tree in comparison to subroot by recursively
        // checking root's l and r subtrees and matching against the sub_root
        let (left, right) = (TreeNode::left(&root), TreeNode::right(&root));

        Self::is_subtree(left, sr2) || Self::is_subtree(right, sr3)
    }

    pub fn is_same(n1: Option<TreeNodeRef>, n2: Option<TreeNodeRef>) -> bool {
        if n1 == None && n2 == None { // both None (same)
            return true
        }

        if n1 == None || n2 == None { // only one is None (not same)
            return false
        }

        // If the current node value matches, check the that the l & r subtrees both match, recursively
        if TreeNode::value(&n1) == TreeNode::value(&n2) {
            return Self::is_same(TreeNode::left(&n1), TreeNode::left(&n2)) &&
                Self::is_same(TreeNode::right(&n1), TreeNode::right(&n2))
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

