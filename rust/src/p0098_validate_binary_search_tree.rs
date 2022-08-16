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

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> TreeNodeRef {
        Rc::new(
            RefCell::new(
                TreeNode {
                    val,
                    left: None,
                    right: None
                }
            )
        )
    }
}


use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn is_valid_bst(root: &Option<TreeNodeRef>) -> bool {
        Solution::helper(root.as_ref().map(Rc::clone), i32::MIN, i32::MAX)
    }

    pub fn helper(node: Option<TreeNodeRef>, min: i32, max: i32) -> bool {
        // true is somewhat base case, until we've traveresed through the entire path down to leaf node 
        if node == None {
            return true
        }

        // grab value
        let value = node.as_ref().unwrap().borrow().val;

        // shortcut early if not within proper BST ranges
        if value < min || value > max {
            return false
        }

        // if go left, current node's value is going to be max (everything on left is smaller than current)
        // if go right, current node's value is going to be min (everything on right is bigger than current)
        Solution::helper(node.as_ref().unwrap().borrow().left.as_ref().map(Rc::clone), min, value) &&
            Solution::helper(node.as_ref().unwrap().borrow().right.as_ref().map(Rc::clone), value, max)

    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

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
