/*
450. Delete Node in a BST
Medium

Given a root node reference of a BST and a key, delete the node with the given key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:

    Search for a node to remove.
    If the node is found, delete the node.

Follow up: Can you solve it with time complexity O(height of tree)?

 

Example 1:

Input: root = [5,3,6,2,4,null,7], key = 3
Output: [5,4,6,2,null,null,7]
Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.

Example 2:

Input: root = [5,3,6,2,4,null,7], key = 0
Output: [5,3,6,2,4,null,7]
Explanation: The tree does not contain a node with value = 0.

Example 3:

Input: root = [], key = 0
Output: []

 

Constraints:

    The number of nodes in the tree is in the range [0, 104].
    -105 <= Node.val <= 105
    Each node has a unique value.
    root is a valid binary search tree.
    -105 <= key <= 105


 */

#[path = "./p0108_convert_sorted_array_to_bst.rs"] pub mod bst;
use bst::Solution as util;

type TreeNode = bst::TreeNode;

pub struct Solution {}

/*
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }
 
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
*/

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // Keep a separate variable so we avoid already borrowed errors when doing two successive borrowing in one line!
        let new_subtree;

        match root {
            None => None,
            // Unwrap here to save us from having to do it in the rest of the code numerous times,
            Some(node) => {
                if key < node.borrow().val { // Ensure our left child is prepared that the left subtree may be updated
                    new_subtree = Solution::delete_node(node.borrow_mut().left.clone(), key);
                    node.borrow_mut().left = new_subtree;
                } else if key > node.borrow().val {  // Ensure our right child is prepared that the right subtree may be updated
                    new_subtree = Solution::delete_node(node.borrow_mut().right.clone(), key);
                    node.borrow_mut().right = new_subtree;
                } else { // Found key
                    // If we have found key but have no left subtree, remove the node at key by returning the right subtree
                    // (this will eclipse/drop the current node)
                    if node.borrow().left.is_none() {
                        return node.borrow_mut().right.clone();
                    } else if node.borrow().right.is_none() {
                        // If we have found key but have no right subtree,
                        // Remove the node at key by returning the left subtree, (this will eclipse/drop the current node)
                        return node.borrow_mut().left.clone();
                    }

                    // We have found the key and we have both left and right subtrees, we need to substitute the key with the next largest value

                    // Essentially we pluck the minimum value in right subtree
                    // (which is the leftmost value in the right BST subtree, extract that value and delete that physical node)
                    let mut minimum = node.borrow_mut().right.clone();

                    while let Some(temp) = minimum.clone().unwrap().borrow_mut().left.clone() {
                        minimum = temp.borrow_mut().left.clone();
                    }

                    // Now that we have minimum extract its key and use it as our replacement
                    // (in this case we are not deleting the node but just replacing the i32 val)
                    let min_key = minimum.unwrap().borrow().val;
                    node.borrow_mut().val = min_key;

                    // Recursively traverse to delete the physical node attached to the min_key since we are using it elsewhere!
                    new_subtree = Solution::delete_node(node.borrow_mut().right.clone(), min_key);
                    node.borrow_mut().right = new_subtree;
                }

                // Rewrap
                Some(node)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0450() {
        //Case 1)
        let mut input = [5,3,6,2,4,7];
        input.sort();
        let root = util::sorted_array_to_bst(&input); // reuse don't have to rewrite!
        let new_root = Solution::delete_node(root, 3);

        let mut check = vec![];
        // Verify Solution root via inorder traversal is the same as the input!
        util::inorder(new_root, &mut check);
        assert_eq!(vec![2,4,5,6,7], check);

        //Case 2)
        let mut input = [5,3,6,2,4,7];
        input.sort();
        let root = util::sorted_array_to_bst(&input);
        let new_root = Solution::delete_node(root, 0);

        let mut check = vec![];
        // Verify Solution root via inorder traversal is the same as the input!
        util::inorder(new_root, &mut check);
        assert_eq!(vec![2,3,4,5,6,7], check);


        //Case 3)
        let root = util::sorted_array_to_bst(&[]);
        let new_root = Solution::delete_node(root, 0);
        assert_eq!(None, new_root);
    }
}

/*

original root is Some(
    RefCell {
        value: TreeNode {
            val: 5,
            left: Some(
                RefCell {
                    value: TreeNode {
                        val: 3,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 2,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 4,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                    },
                },
            ),
            right: Some(
                RefCell {
                    value: TreeNode {
                        val: 7,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 6,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: None,
                    },
                },
            ),
        },
    },
)
new root is Some(
    RefCell {
        value: TreeNode {
            val: 5,
            left: Some(
                RefCell {
                    value: TreeNode {
                        val: 4,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 2,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: None,
                    },
                },
            ),
            right: Some(
                RefCell {
                    value: TreeNode {
                        val: 7,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 6,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: None,
                    },
                },
            ),
        },
    },
)


 */
