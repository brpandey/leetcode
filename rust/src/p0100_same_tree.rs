

use crate::util::TreeNode;
use crate::util::TreeNodeRef;

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(p: &Option<TreeNodeRef>, q: &Option<TreeNodeRef>) -> bool {

        // Base case, if both leaves are empty, return true
        if p.is_none() && q.is_none() {
            return true
        }

        if p.is_none() || q.is_none() {
            return false
        }
        
        // Recursive case
        // ensure node values are the same
        if p.is_some() && q.is_some() && TreeNode::value(p) == TreeNode::value(q) {
            let (left_p, left_q) = (TreeNode::left(p), TreeNode::left(q));
            let (right_p, right_q) = (TreeNode::right(p), TreeNode::right(q));

            // check subtrees
            return Solution::is_same_tree(&left_p, &left_q) // check left subtrees
                && Solution::is_same_tree(&right_p, &right_q) // check right subtrees
        }

        return false
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    pub fn test_0100() {

        let p_l = TreeNode::new(2);
        let p_r = TreeNode::new(3);
        let p = Some(TreeNode::new_with(1, Some(Rc::clone(&p_l)), Some(Rc::clone(&p_r))));

        let q_l = TreeNode::new(2);
        let q_r = TreeNode::new(3);
        let q = Some(TreeNode::new_with(1, Some(Rc::clone(&q_l)), Some(Rc::clone(&q_r))));

        assert_eq!(true, Solution::is_same_tree(&p, &q));

        let p_l = TreeNode::new(2);
        let p = Some(TreeNode::new_with(1, Some(Rc::clone(&p_l)), None));

        let q_r = TreeNode::new(2);
        let q = Some(TreeNode::new_with(1, None, Some(Rc::clone(&q_r))));

        assert_eq!(false, Solution::is_same_tree(&p, &q));

        let p_l = TreeNode::new(2);
        let p_r = TreeNode::new(1);
        let p = Some(TreeNode::new_with(1, Some(Rc::clone(&p_l)), Some(Rc::clone(&p_r))));

        let q_l = TreeNode::new(1);
        let q_r = TreeNode::new(2);
        let q = Some(TreeNode::new_with(1, Some(Rc::clone(&q_l)), Some(Rc::clone(&q_r))));

        assert_eq!(false, Solution::is_same_tree(&p, &q));

    }
}

