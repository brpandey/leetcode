// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }
 
 impl TreeNode {
   #[inline]
     pub fn new(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Rc<RefCell<TreeNode>> {
       Rc::new(
           RefCell::new(
               TreeNode {
                   val,
                   left,
                   right,
               }
           )
       )
     }

     pub fn value(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
         node.as_ref().unwrap().borrow().val
     }

     pub fn left(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
         node.as_ref().unwrap().borrow().left.as_ref().map(|l| Rc::clone(l))
     }

     pub fn right(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
         node.as_ref().unwrap().borrow().right.as_ref().map(|r| Rc::clone(r))
     }
 }

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn is_same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {

        // Base case, if leaves are empty, return true
        if *p == None && *q == None {
            return true
        }

        // Recursive case
        if *p != None && *q != None && TreeNode::value(p) == TreeNode::value(q) {
            let l1 = TreeNode::left(p);
            let l2 = TreeNode::left(q);

            let r1 = TreeNode::right(p);
            let r2 = TreeNode::right(q);

            return Solution::is_same_tree(&l1,&l2) && Solution::is_same_tree(&r1,&r2)
        }

        return false
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0100() {

        let p_l = TreeNode::new(2, None, None);
        let p_r = TreeNode::new(3, None, None);
        let p = Some(TreeNode::new(1, Some(Rc::clone(&p_l)), Some(Rc::clone(&p_r))));

        let q_l = TreeNode::new(2, None, None);
        let q_r = TreeNode::new(3, None, None);
        let q = Some(TreeNode::new(1, Some(Rc::clone(&q_l)), Some(Rc::clone(&q_r))));

        assert_eq!(true, Solution::is_same_tree(&p, &q));

        let p_l = TreeNode::new(2, None, None);
        let p = Some(TreeNode::new(1, Some(Rc::clone(&p_l)), None));

        let q_r = TreeNode::new(2, None, None);
        let q = Some(TreeNode::new(1, None, Some(Rc::clone(&q_r))));

        assert_eq!(false, Solution::is_same_tree(&p, &q));


        let p_l = TreeNode::new(2, None, None);
        let p_r = TreeNode::new(1, None, None);
        let p = Some(TreeNode::new(1, Some(Rc::clone(&p_l)), Some(Rc::clone(&p_r))));

        let q_l = TreeNode::new(1, None, None);
        let q_r = TreeNode::new(2, None, None);
        let q = Some(TreeNode::new(1, Some(Rc::clone(&q_l)), Some(Rc::clone(&q_r))));

        assert_eq!(false, Solution::is_same_tree(&p, &q));

    }
}

