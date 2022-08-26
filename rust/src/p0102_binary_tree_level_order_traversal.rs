/*
 * 102. Binary Tree Level Order Traversal
Medium

Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

 

Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: [[3],[9,20],[15,7]]

Example 2:

Input: root = [1]
Output: [[1]]

Example 3:

Input: root = []
Output: []


Constraints:

    The number of nodes in the tree is in the range [0, 2000].
    -1000 <= Node.val <= 1000


 *
 */

use std::collections::VecDeque;

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn level_order(root: Option<TreeNodeRef>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        queue.push_back(root);

        while !queue.is_empty() {

            let size = queue.len();
            let mut list = vec![];

            for _i in 0..size {
                let node = queue.pop_front().unwrap();

                if node.is_some() {
                    list.push(TreeNode::value(&node));

                    let l = TreeNode::left(&node);
                    let r = TreeNode::right(&node);

                    if l.is_some() { queue.push_back(l) }
                    if r.is_some() { queue.push_back(r) }
                }
            }

            result.push(list);
        }

        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0102() {
        let root = TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
        assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], Solution::level_order(root));
    }
}

