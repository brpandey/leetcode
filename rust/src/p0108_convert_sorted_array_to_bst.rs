/*
108. Convert Sorted Array to Binary Search Tree
Easy

Given an array where elements are sorted in ascending order, convert it to a height balanced BST.
For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.

Example:

Given the sorted array: [-10,-3,0,5,9],
One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:

0
/ \
-3   9
/   /
-10  5

 */

use crate::util::TreeNodeRef;
use crate::util::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: &[i32]) -> Option<TreeNodeRef> {
        let size = nums.len();

        // [], return None
        // [1], size is 1, mid is 0, left is [0..0] or [], right is [1..] or []
        // [1, 2], size is 2, mid is 1, left is [0..1] or [1], right is [2..] or []
        // [1, 2, 3], size is 3, mid is 1, left is [0..1] or [1,2], right is [2..] or [3]
        // [1, 2, 3, 4], size is 4, mid is 2, left is [0..2] or [1,2], right is [3..] or [4]
        // [1, 2, 3, 4, 5], size is 5, mid is 2, left is [0..2] or [1,2], right is [3..] or [4, 5]

        if size == 0 { return None };
        let mid = size/2;

        // Put the middle element of the slice as the root
        let root: TreeNodeRef = TreeNode::new(nums[mid]);
        root.borrow_mut().left = Solution::sorted_array_to_bst(&nums[..mid]);
        root.borrow_mut().right = Solution::sorted_array_to_bst(&nums[mid+1..]);

        Some(root)
    }

    // Helper function to verify tree is constructed properly
    pub fn inorder(node: Option<TreeNodeRef>, nums: &mut Vec<i32>) {
        if node == None {
            return;
        }

        let left = TreeNode::left(&node);
        let right = TreeNode::right(&node);
        let value = TreeNode::value(&node);

        // Inorder is LNR
        Solution::inorder(left, nums);
        nums.push(value);
        Solution::inorder(right, nums);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0108() {
        let input = vec![-10,-3,0,5,9];
        let root = Solution::sorted_array_to_bst(&input);
        let mut check = vec![];

        // Verify Solution root via inorder traversal is the same as the input!
        Solution::inorder(root, &mut check);
        assert_eq!(input, check);
    }
}

/*
root is RefCell {
    value: TreeNode {
        val: 0,
        left: Some(
            RefCell {
                value: TreeNode {
                    val: -3,
                    left: Some(
                        RefCell {
                            value: TreeNode {
                                val: -10,
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
                    val: 9,
                    left: Some(
                        RefCell {
                            value: TreeNode {
                                val: 5,
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
}

 */
