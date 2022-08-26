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


use std::cmp;

use crate::util::TreeNode;
use crate::util::TreeNodeRef as TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn max_path_sum(root: Option<TreeNodeRef>) -> i32 {
        let (mf, ms) = Self::dfs(root);
        cmp::max(mf, ms)
    }

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

    pub fn dfs(root: Option<TreeNodeRef>) -> (i32, i32) {
        if root == None {
            return (0, 0)
        }

        let (l_mf, l_ms) = Self::dfs(TreeNode::left(&root));
        let (r_mf, r_ms) = Self::dfs(TreeNode::right(&root));

        let mf = cmp::max(l_mf, r_mf);

        let val = TreeNode::value(&root);

        // Compute 1) and 2)
        let max_forked = cmp::max(mf, val + l_ms + r_ms); // update max_forked with the forked value at this level
        let max_single = val + cmp::max(l_ms, r_ms); // find max single at this level will it be from l or r path?

        (max_forked, cmp::max(max_single, 0))
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

