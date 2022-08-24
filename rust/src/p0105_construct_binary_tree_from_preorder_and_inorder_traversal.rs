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

use std::collections::VecDeque;

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


    // Create tree structure from bfs list of values
    pub fn from_list(list: Vec<i32>) -> Option<TreeNodeRef> {
//      TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
        let mut queue1 = list.iter().cloned().collect::<VecDeque<i32>>();
        let mut queue2: VecDeque<TreeNodeRef> = VecDeque::new();

        let head = TreeNode::new(queue1.pop_front().unwrap());
        let mut node;

        queue2.push_back(Rc::clone(&head));

        let mut value: i32;
        let mut left = true;
        let mut child: TreeNodeRef;

        // Process bfs value queue while processing bfs node queue
        while !queue1.is_empty() {
            node = queue2.front_mut().unwrap();
            value = queue1.pop_front().unwrap();

            // handle where we need to not add child nodes to current node by continue-ing
            if value == i32::MIN {
                left = !left;

                if left == false {
                    queue2.pop_front(); // if we have a null right child ensure we clear current node value from queue2
                }
                continue
            }

            // Create proper node out of value 
            child = TreeNode::new(value);

            // Connect the child nodes either left or right child,
            // Enqueuing the child for its later processing as a potential parent
            // left
            if left {
                node.borrow_mut().left = Some(Rc::clone(&child));
                queue2.push_back(Rc::clone(&child));
            } else { // right
                node.borrow_mut().right = Some(Rc::clone(&child));
                queue2.push_back(Rc::clone(&child));
                queue2.pop_front(); // now that both l and r children have been assigned, pop parent
            }

            left = !left;
        }

        Some(head)
    }

}

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

use std::rc::Rc;
use std::cell::RefCell;

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
        let tree_output = TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
//        dbg!(&tree_output);
        assert_eq!(root, tree_output);

        let root = Solution::build_tree(&vec![-1], &vec![-1]);
        let tree_output = TreeNode::from_list(vec![-1]);
//        dbg!(&tree_output);

        assert_eq!(root, tree_output);

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
