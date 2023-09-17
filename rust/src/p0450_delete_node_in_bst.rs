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

/*
             5
            / \
           3   6
          / \   \
         2   4   7

key to delete is 3, 
*/


use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn delete_node(root: Option<TreeNodeRef>, key: i32) -> Option<TreeNodeRef> {
        let subtree;
        let n = &root;

        if n.is_none() { return None };

        let value = TreeNode::value(n);
        let left = TreeNode::left(n);
        let mut right = TreeNode::right(n);

        if key < value { // Update left with new subtree 
            subtree = Solution::delete_node(left, key);
            n.as_ref().unwrap().borrow_mut().left = subtree;
        } else if key > value {  // Update right with new subtree
            subtree = Solution::delete_node(right, key);
            n.as_ref().unwrap().borrow_mut().right = subtree;
        } else { // Found key
            // Case 1) no left subtree, delete node by turning right subtree, eclipsing current
            if left.is_none() {
                return right
            // Case 2) no left subtree, delete node by turning right subtree, eclipsing current
            } else if right.is_none() {
                return left
            }

            // If both left or right is None, we return that in each of the return right or return left above

            // Case 3) Key is found with node having both left and right subtrees,
            // Hence substitute the key with the next largest value
            // AKA => Grab minimum value in RIGHT subtree
            // (which is the leftmost value in the right BST subtree, extract that value and delete that physical node)
            let mut smallest = right;
            let mut smallest_right = -1;

            while smallest.is_some() {
                smallest_right = TreeNode::value(&smallest);
                smallest = TreeNode::left(&smallest); // find smallest in right subtree
            }

            // Now that we have the next smallest and its key, use it as our replacement

            n.as_ref().unwrap().borrow_mut().data = smallest_right;

            // Recursively traverse to delete the physical node attached to the smallest_right since we are using it elsewhere!
            right = TreeNode::right(n);
            subtree = Solution::delete_node(right, smallest_right);
            n.as_ref().unwrap().borrow_mut().right = subtree;
        }

        n.clone()
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
        let root = TreeNode::sorted_array_to_bst(&input); // reuse don't have to rewrite!
        let new_root = Solution::delete_node(root, 3);

        let mut check = vec![];
        TreeNode::inorder(new_root, &mut check);
        assert_eq!(vec![2,4,5,6,7], check);

        //Case 2)
        let mut input = [5,3,6,2,4,7];
        input.sort();
        let root = TreeNode::sorted_array_to_bst(&input);
        let new_root = Solution::delete_node(root, 0);

        let mut check = vec![];
        TreeNode::inorder(new_root, &mut check);
        assert_eq!(vec![2,3,4,5,6,7], check);


        //Case 3)
        let root = TreeNode::sorted_array_to_bst(&[]);
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
