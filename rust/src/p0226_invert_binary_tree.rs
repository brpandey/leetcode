/*
 *
 * 226. Invert Binary Tree
Easy

Given the root of a binary tree, invert the tree, and return its root.

Example 1:

Input: root = [4,2,7,1,3,6,9]
Output: [4,7,2,9,6,3,1]

Example 2:

Input: root = [2,1,3]
Output: [2,3,1]

Example 3:

Input: root = []
Output: []

Constraints:

    The number of nodes in the tree is in the range [0, 100].
    -100 <= Node.val <= 100

 */

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        if root == None { return None }

        let left = TreeNode::left(&root);
        let right = TreeNode::right(&root);

        root.as_ref().unwrap().borrow_mut().left = Solution::invert_tree(right);
        root.as_ref().unwrap().borrow_mut().right = Solution::invert_tree(left);

        root
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    pub fn test_0226() {
        // case 1
        let n2 = TreeNode::new(2);
        let n1 = TreeNode::new(1);
        let n3 = TreeNode::new(3);

        n2.borrow_mut().left = Some(n1);
        n2.borrow_mut().right = Some(n3);

        let head = Some(n2);

        let _inverted = Solution::invert_tree(head);

//        dbg!(inverted);

        // case 2
        let n4 = TreeNode::new(4);
        let n2 = TreeNode::new(2);
        let n7 = TreeNode::new(7);
        let n1 = TreeNode::new(1);
        let n3 = TreeNode::new(3);
        let n6 = TreeNode::new(6);
        let n9 = TreeNode::new(9);

        n4.borrow_mut().left = Some(Rc::clone(&n2));
        n4.borrow_mut().right = Some(Rc::clone(&n7));

        n2.borrow_mut().left = Some(n1);
        n2.borrow_mut().right = Some(n3);

        n7.borrow_mut().left = Some(n6);
        n7.borrow_mut().right = Some(n9);

        let head = Some(n4);

        let _inverted = Solution::invert_tree(head);

//       dbg!(inverted);

        // case 3
        assert_eq!(None, Solution::invert_tree(None));
    }
}

