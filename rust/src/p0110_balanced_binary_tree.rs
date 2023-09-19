/*
110. Balanced Binary Tree
Easy

Given a binary tree, determine if it is
height-balanced

Example 1:

         3
        / \
       9  20
          /\
         15 7

(height differs by 1 level at most => so true)


Input: root = [3,9,20,null,null,15,7]
Output: true
*/

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(root.clone()).0
    }

    pub fn dfs(node: Option<TreeNodeRef>) -> (bool, i32) {
        if node.is_none() {
            // tuple.0 is if balanced t/f,
            // tuple.1 is height
            return (true, 0)
        }

        let l = Self::dfs(TreeNode::left(&node));

        if !l.0 { return (false, 0) } // optimization to avoid going down right subtree

        let r = Self::dfs(TreeNode::right(&node));

        // current node is balanced if its subtrees are balanced
        // and the height difference between them is not more than 1
        let height = 1 + std::cmp::max(l.1, r.1);

        let balanced = l.0 && r.0 && (l.1 - r.1).abs() <= 1;
        (balanced, height)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0XXX::tests::test_0XXX

    #[test]
    pub fn test_0110() {
        let root = TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
        assert_eq!(true, Solution::is_balanced(root));

        let root = TreeNode::from_list(vec![1,2,2,3,3,i32::MIN,i32::MIN,4,4]);
        assert_eq!(false, Solution::is_balanced(root));

        assert_eq!(true, Solution::is_balanced(None));
    }
}

