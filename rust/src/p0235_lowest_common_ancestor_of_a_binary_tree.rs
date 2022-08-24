/*
 * 235. Lowest Common Ancestor of a Binary Search Tree
Medium

Given a binary search tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

 

Example 1:

Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
Output: 3
Explanation: The LCA of nodes 5 and 1 is 3.

Example 2:

Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
Output: 5
Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.

Example 3:

Input: root = [1,2], p = 1, q = 2
Output: 1

 

Constraints:

    The number of nodes in the tree is in the range [2, 105].
    -109 <= Node.val <= 109
    All Node.val are unique.
    p != q
    p and q will exist in the tree.


 *
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

    pub fn value(node: &Option<TreeNodeRef>) -> i32 {
        node.as_ref().unwrap().borrow().val
    }

    pub fn left(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        node.as_ref().unwrap().borrow().left.as_ref().map(Rc::clone)
    }

    pub fn right(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        node.as_ref().unwrap().borrow().right.as_ref().map(Rc::clone)
    }
}

/*
Strategy

  if p and q are both less than the current node we know that they must be in the left subtree in a BST
  also
  if p and q are both greater than the current node we know that they must be in the right subtree in a BST

  if p and q are split into different subtrees that means p is less than current node while q is not or vice versa
  OR p or q equals the current node, we then know that the current node is the node of the split or self-referential
  lowest common ancestor where p and q are descendants

*/

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn lowest_common_ancestor(root: Option<TreeNodeRef>, p: Option<TreeNodeRef>, q: Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        let mut node: Option<TreeNodeRef> = root;

        while None != node {
            if TreeNode::value(&p) < TreeNode::value(&node) && TreeNode::value(&q) < TreeNode::value(&node) {
                node = TreeNode::left(&node);
            }
            else if TreeNode::value(&p) > TreeNode::value(&node) && TreeNode::value(&q) > TreeNode::value(&node) {
                node = TreeNode::right(&node);
            }
            else {
                return node
            }
        }

        return None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0236() {
        // Input [6,2,8,0,4,7,9,null,null,3,5] 

        let node0 = TreeNode::new(0);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);
        let node6 = TreeNode::new(6);
        let node7 = TreeNode::new(7);
        let node8 = TreeNode::new(8);
        let node9 = TreeNode::new(9);

        node6.borrow_mut().left = Some(Rc::clone(&node2));
        node6.borrow_mut().right = Some(Rc::clone(&node8));

        node2.borrow_mut().left = Some(Rc::clone(&node0));
        node2.borrow_mut().right = Some(Rc::clone(&node4));

        node8.borrow_mut().left = Some(Rc::clone(&node7));
        node8.borrow_mut().right = Some(Rc::clone(&node9));

        node4.borrow_mut().left = Some(Rc::clone(&node3));
        node4.borrow_mut().right = Some(Rc::clone(&node5));

        let root = Some(Rc::clone(&node6));

        let result = Solution::lowest_common_ancestor(root, Some(Rc::clone(&node2)), Some(Rc::clone(&node8)));
        assert_eq!(6, result.unwrap().borrow().val);


        let root = Some(Rc::clone(&node6));

        let result = Solution::lowest_common_ancestor(root, Some(Rc::clone(&node2)), Some(Rc::clone(&node4)));
        assert_eq!(2, result.unwrap().borrow().val);

    }
}

