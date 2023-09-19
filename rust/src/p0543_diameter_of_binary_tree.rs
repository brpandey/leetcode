/*
543. Diameter of Binary Tree
Easy
12.5K
782
Companies

Given the root of a binary tree, return the length of the diameter of the tree.

The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

The length of a path between two nodes is represented by the number of edges between them.

             1         <-- diameter for 1 is h(2) + h(3) or 2+1 or 3
            / \
           2   3       <-- diameter for 2 is h(4) + h(5) or 1+1 or 2, diameter for 3 is 0
          / \
         4   5         <-- diameter here is 0

Example 1:

Input: root = [1,2,3,4,5]
Output: 3
Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].

*/

use std::rc::Rc;
use std::cell::RefCell;
use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(root.clone()).0
    }

    pub fn dfs(node: Option<TreeNodeRef>) -> (i32, i32) {
        use std::cmp;

        if node.is_none() {
            //tuple.0 is diameter,
            //tuple.1 is height
            return (0, 0)
        }

        let l = Self::dfs(TreeNode::left(&node));
        let r = Self::dfs(TreeNode::right(&node));

        let max_diameter = cmp::max(l.0, r.0);
        let acc_max_diameter = cmp::max(max_diameter, l.1 + r.1); // notice, diameter is the edges between the nodes

        let max_height = 1 + cmp::max(l.1, r.1); // if this was a leave node, max => 0 and adding 1 gives 1
        (acc_max_diameter, max_height)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0543_diameter_of_binary_tree::tests::test_0543

    #[test]
    pub fn test_0543() {
        let root = TreeNode::from_list(vec![1,2,3,4,5]);
        assert_eq!(3, Solution::diameter_of_binary_tree(root));
    }
}

