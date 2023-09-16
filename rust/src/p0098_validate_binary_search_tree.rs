/*
98. Validate Binary Search Tree
Medium

Given the root of a binary tree, determine if it is a valid binary search tree (BST).

A valid BST is defined as follows:

The left subtree of a node contains only nodes with keys less than the node's key.
The right subtree of a node contains only nodes with keys greater than the node's key.
Both the left and right subtrees must also be binary search trees.



Example 1:

Input: root = [2,1,3]
Output: true

Example 2:

Input: root = [5,1,4,null,null,3,6]
Output: false
Explanation: The root node's value is 5 but its right child's value is 4.

Constraints:

The number of nodes in the tree is in the range [1, 104].
-231 <= Node.val <= 231 - 1

*/

use crate::util::TreeNodeRef;
use crate::util::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: &Option<TreeNodeRef>) -> bool {
        Solution::dfs(root.clone(), i32::MIN, i32::MAX)
    }

    pub fn dfs(node: Option<TreeNodeRef>, min: i32, max: i32) -> bool {
        // true is somewhat base case, until we've traveresed through the entire path down to leaf node 
        if node.is_none() { return true }
        let value = TreeNode::value(&node);

        if value < min || value > max {
            // if not within proper BST ranges, return false
            return false
        }

        // Case 1) if go left, current node's value is going to be max (everything on left is smaller than current)
        // Case 2) if go right, current node's value is going to be min (everything on right is bigger than current)
        let (left, right) = (TreeNode::left(&node), TreeNode::right(&node));

        // DFS both branches and must be true for both recursive calls
        Solution::dfs(left, min, value) && Solution::dfs(right, value, max)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use std::rc::Rc;
    
    #[test]
    pub fn test_0098() {
        // case 1
        let n2 = TreeNode::new(2);
        let n1 = TreeNode::new(1);
        let n3 = TreeNode::new(3);

        n2.borrow_mut().left = Some(n1);
        n2.borrow_mut().right = Some(n3);

        let head = Some(n2);

        assert_eq!(true, Solution::is_valid_bst(&head));

  //      dbg!(head);


        //  5
        //1   4
        //   3 6


        let n5 = TreeNode::new(5);
        let n1 = TreeNode::new(1);
        let n4 = TreeNode::new(4);
        let n3 = TreeNode::new(3);
        let n6 = TreeNode::new(6);

        n5.borrow_mut().left = Some(n1);
        n5.borrow_mut().right = Some(Rc::clone(&n4));

        n4.borrow_mut().left = Some(n3);
        n4.borrow_mut().right = Some(n6);

        let head = Some(n5);

        assert_eq!(false, Solution::is_valid_bst(&head));

//        dbg!(head);

    }
}

/*
src/p0098_validate_binary_search_tree.rs:132] head = Some(
    RefCell {
        value: TreeNode {
            val: 5,
            left: Some(
                RefCell {
                    value: TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    },
                },
            ),
            right: Some(
                RefCell {
                    value: TreeNode {
                        val: 4,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 3,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 6,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                    },
                },
            ),
        },
    },
)
*/
