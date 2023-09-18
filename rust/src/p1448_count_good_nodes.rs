/*

1448. Count Good Nodes in Binary Tree
Medium

Given a binary tree root, a node X in the tree is named good if in the path from root to X there
are no nodes with a value greater than X.

Return the number of good nodes in the binary tree.


           3*
          / \
         1   4*
        /   / \
       3*  1   5*

    4 asterisks => 4 good nodes that are all larger than preceding value on path
*/

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        println!("nodes are {:?}", &root);
        Self::dfs(root.clone(), i32::MIN)
    }

    pub fn dfs(node: Option<TreeNodeRef>, mut acc_max: i32) -> i32 {
        if node.is_none() { return 0 }

        let mut count = 0;
        let value = TreeNode::value(&node);

        if value >= acc_max {
            println!("value {} is >= acc_max {}", &value, &acc_max);
            count = 1;
            acc_max = value;
        }

        count += Self::dfs(TreeNode::left(&node), acc_max);
        count += Self::dfs(TreeNode::right(&node), acc_max);

        count
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0XXX::tests::test_0XXX

    #[test]
    pub fn test_1448() {
        let root = TreeNode::from_list(vec![3,1,4,3,i32::MIN,1,5]);
        assert_eq!(4, Solution::good_nodes(root));
    }
}

