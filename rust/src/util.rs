use std::rc::Rc;
use std::cell::RefCell;
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

    pub fn value(node: &Option<TreeNodeRef>) -> i32 {
        node.as_ref().unwrap().borrow().val
    }

    pub fn left(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        node.as_ref().unwrap().borrow().left.as_ref().map(Rc::clone)
    }

    pub fn right(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        node.as_ref().unwrap().borrow().right.as_ref().map(Rc::clone)
    }

    
    // Create tree structure from bfs list of values
    pub fn from_list(list: Vec<i32>) -> Option<TreeNodeRef> {
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
