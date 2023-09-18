/*
 *
 * 124. Binary Tree Maximum Path Sum
Hard

A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.

The path sum of a path is the sum of the node's values in the path.

Given the root of a binary tree, return the maximum path sum of any non-empty path.

Example 1:

Input: root = [1,2,3]
Output: 6
Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.

Example 2:

Input: root = [-10,9,20,null,null,15,7]
Output: 42
Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.

Constraints:

    The number of nodes in the tree is in the range [1, 3 * 104].
    -1000 <= Node.val <= 1000

 *
 */

/*
 * Strategy:
 *
 * At each level of the tree track the max forked value and the mark single path value
 */

// Adding 1) val + max l + max r gives the value over the path LNR which is branched/forked over N
//
//            N       whereas 2) val + max(l, r)  gives either   N      or      N
//           / \                                                /                \
//          L   R                                              L                  R

use std::cmp;
use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<TreeNodeRef>) -> i32 {
        let (max_split, max_nosplit) = Self::dfs(root);
        cmp::max(max_split, max_nosplit)
    }

    pub fn dfs(node: Option<TreeNodeRef>) -> (i32, i32) {
        if node == None {
            // tuple.0 is max path sum with split,
            // tuple.1 is max path sum without split
            return (0, 0)
        }

        let left_max = Self::dfs(TreeNode::left(&node));
        let right_max = Self::dfs(TreeNode::right(&node));

        let val = TreeNode::value(&node);

        // Compute 1) and 2)
        let mut acc_max_with_split = cmp::max(left_max.0, right_max.0);
        acc_max_with_split = cmp::max(acc_max_with_split, val + left_max.1 + right_max.1);  // find the bigger max path split

        let acc_max_without_split  = cmp::max(0, val + cmp::max(left_max.1, right_max.1));
        (acc_max_with_split, acc_max_without_split)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0124() {
        let root = TreeNode::from_list(vec![1,2,3]);
        assert_eq!(6, Solution::max_path_sum(root));

        let root = TreeNode::from_list(vec![-10,9,20,i32::MIN,i32::MIN,15,7]);
        assert_eq!(42, Solution::max_path_sum(root)); 
    }
}
