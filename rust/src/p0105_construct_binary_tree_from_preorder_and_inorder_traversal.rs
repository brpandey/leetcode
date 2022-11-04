/*
 * 105. Construct Binary Tree from Preorder and Inorder Traversal
Medium

Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

Example 1:

Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
Output: [3,9,20,null,null,15,7]

Example 2:

Input: preorder = [-1], inorder = [-1]
Output: [-1]

Constraints:

    1 <= preorder.length <= 3000
    inorder.length == preorder.length
    -3000 <= preorder[i], inorder[i] <= 3000
    preorder and inorder consist of unique values.
    Each value of inorder also appears in preorder.
    preorder is guaranteed to be the preorder traversal of the tree.
    inorder is guaranteed to be the inorder traversal of the tree.
 */

use crate::util::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

/*
Strategy:
Build tree recursively, think only one level at a time

The goal for the current level is just to find the root and partition each of the preorder and inorder lists into their
constituent left and right halves

Key Observations:
First element in preorder is always the root at that level
Everything before root element in inorder list is the left subtree, so root element index represents size of left subtree
//
//             0 1  2  3 4              0 1  2  3 4
// preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//               - -------              -   -------
//  NLR          L    R       LNR       L      R
//

*/

pub struct Solution {}

impl Solution {
    pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() != inorder.len() || preorder.is_empty() || inorder.is_empty() {
            return None
        }

        // Find root node, in preorder it's the first element e.g. preorder = NLR
        let root = preorder[0];
        let root_index = inorder.iter().position(|&v| v == root).unwrap();

        // Given inorder = LNR, we know that 0..root_index is left subtree, and root_index+1.. is right subtree (for inorder vec)
        // For preorder, since it is NLR, we know that 0 is root and  1..root_index+1 is the left subtree, we know this because
        // looking at the inorder vector 3 is the root and is at index 1, which says that only 1 element which is 9 (at index 0) is in the left subtree

        // build tree recursively by building subtrees recursively
        let r = TreeNode::new(root);

        // select left subtree portions of preorder and inorder lists, do same for right
        r.as_ref().borrow_mut().left = Self::build_tree(&preorder[1..root_index+1], &inorder[0..root_index]);
        r.as_ref().borrow_mut().right = Self::build_tree(&preorder[root_index+1..], &inorder[root_index+1..]);

        // subtree value is moved out
        Some(r)
    }
}




#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0105() {
        let root = Solution::build_tree(&vec![3,9,20,15,7], &vec![9,3,15,20,7]);
        let check = TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
        assert_eq!(root, check);

        let root = Solution::build_tree(&vec![-1], &vec![-1]);
        let check = TreeNode::from_list(vec![-1]);
        assert_eq!(root, check);
    }
}

/*
running 1 test
[src/p0105_construct_binary_tree_from_preorder_and_inorder_traversal.rs:178] &tree_output = Some(
    RefCell {
        value: TreeNode {
            val: 3,
            left: Some(
                RefCell {
                    value: TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    },
                },
            ),
            right: Some(
                RefCell {
                    value: TreeNode {
                        val: 20,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 15,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 7,
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
[src/p0105_construct_binary_tree_from_preorder_and_inorder_traversal.rs:183] &tree_output = Some(
    RefCell {
        value: TreeNode {
            val: -1,
            left: None,
            right: None,
        },
    },
)
test p0105_construct_binary_tree_from_preorder_and_inorder_traversal::tests::test_0105 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 89 filtered out; finished in 0.01s

*/
