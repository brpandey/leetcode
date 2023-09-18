/*

199. Binary Tree Right Side View

    Given the root of a binary tree, imagine yourself standing on the
right side of it, return the values of the nodes you can see ordered
from top to bottom.
/
    Example 1:

    Input: root = [1,2,3,null,5,null,4]
    Output: [1,3,4]

*/

/*
  Traverse tree in level order (bfs).  capture rightmost value in each level and add to output
  Must traverse left subtree first then right subtree

          1
         / \
        2   3
         \   \
          5   4
*/

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn right_side_view(root: Option<TreeNodeRef>) -> Vec<i32> {
        if root.is_none() { return vec![] }

        let mut output = vec![];
        let mut q = VecDeque::new();

        q.push_back(root.clone());

        let mut size;
        let mut current;
        let mut rightmost = None;

        while !q.is_empty() {
            size = q.len();

            for _ in 0..size {
                current = q.pop_front().flatten();

                rightmost = current.clone();

                let left = TreeNode::left(&current);
                let right = TreeNode::right(&current);

                if left.is_some()  {
                    q.push_back(left);
                }

                if right.is_some() {
                    q.push_back(right);
                }
            }

            // grab the last rightmost value for a level and push its value to output queue
            output.push(TreeNode::value(&rightmost))
        }

        output
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0XXX::tests::test_0XXX

    #[test]
    pub fn test_0199() {
        let root = TreeNode::from_list(vec![1,2,3,i32::MIN,i32::MIN,5, 4]);
//        let root = TreeNode::from_list(vec![1,2,3,i32::MIN,5,i32::MIN,4]);
        assert_eq!(vec![1,3,4], Solution::right_side_view(root))
    }
}

