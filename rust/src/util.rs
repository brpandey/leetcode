use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

pub type TreeNodeRef = Rc<RefCell<TreeNode>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]

pub struct TreeNode {
    pub data: i32,
    pub left: Option<TreeNodeRef>,
    pub right: Option<TreeNodeRef>,
}

impl TreeNode {
    #[inline]
    pub fn new(data: i32) -> TreeNodeRef {
        Rc::new(
            RefCell::new(
                TreeNode {
                    data,
                    left: None,
                    right: None
                }
            )
        )
    }

    pub fn new_with(data: i32, left: Option<TreeNodeRef>, right: Option<TreeNodeRef>) -> TreeNodeRef {
        Rc::new(
            RefCell::new(
                TreeNode {
                    data,
                    left,
                    right,
                }
            )
        )
    }

    pub fn value(node: &Option<TreeNodeRef>) -> i32 {
        if node.is_none() { return i32::MIN }
        node.as_ref().unwrap().borrow().data
    }

    pub fn left(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        if node.is_none() { return None }
        node.as_ref().unwrap().borrow().left.as_ref().map(Rc::clone)
    }

    pub fn right(node: &Option<TreeNodeRef>) -> Option<TreeNodeRef> {
        if node.is_none() { return None }
        node.as_ref().unwrap().borrow().right.as_ref().map(Rc::clone)
    }

    // Create tree structure from bfs list of values
    // let root = TreeNode::from_list(vec![3,9,20,i32::MIN,i32::MIN,15,7]);
    // assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], Solution::level_order(root));

    /*
                  3
                 / \
                9   20
                    / \
                   15  7
     */

    pub fn from_list(list: Vec<i32>) -> Option<TreeNodeRef> {
        if list.is_empty() { return None }

        let mut q: VecDeque<Option<TreeNodeRef>> = VecDeque::new();

        let root = Some(TreeNode::new(list[0]));
        let mut parent = None;

        q.push_back(root.clone());

        let mut left = true;
        let mut child;

        for &value in list.iter().skip(1) {

            if left { // only grab from queue if we are ready to process left child
                parent = q.pop_front().flatten();
            }

            // Create proper node out of value
            if value == i32::MIN {
                left = !left;
                continue // skip child processing of node value since this child is null
            } else {
                child = Some(TreeNode::new(value)); // create child
            }

            // Connect the child nodes either left or right child,
            // Enqueuing the child for its later processing as a potential parent

            if left { // left
                parent.as_ref().unwrap().borrow_mut().left = child.clone();
            } else { // right
                parent.as_ref().unwrap().borrow_mut().right = child.clone();
            }

            q.push_back(child);
            left = !left; // toggle left so if left was true it is now false, and vice versa
        }


        root
    }

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
        root.borrow_mut().left = TreeNode::sorted_array_to_bst(&nums[..mid]);
        root.borrow_mut().right = TreeNode::sorted_array_to_bst(&nums[mid+1..]);

        Some(root)
    }

    pub fn inorder(node: Option<TreeNodeRef>, nums: &mut Vec<i32>) {
        if node == None {
            return;
        }

        let left = TreeNode::left(&node);
        let right = TreeNode::right(&node);
        let value = TreeNode::value(&node);

        // Inorder is LNR
        TreeNode::inorder(left, nums);
        nums.push(value);
        TreeNode::inorder(right, nums);
    }


}


/* Box List Node */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub data: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new0(data: i32) -> Self {
        ListNode {
            next: None,
            data
        }
    }

    pub fn new(data: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            next: None,
            data
        }))
    }


    pub fn to_list(a: &[u32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        // Reverse the array list [4,5] to 5,4 so 4 then points to 5 etc..
        for &v in a.iter().rev() {
            let mut n = (ListNode::new(v as i32)).unwrap(); // un-wrap
            n.next = head;
            head = Some(n); // re-wrap
        }

        head
    }
}

/* Shared List Node*/

/*
 *  Think of the Option/Rc/RefCell/ListNode as a stack
 *
 *  Option     as_ref, unwrap
 *  -------
 *  Rc         clone (non-thread safe reference counting, over immutable data)
 *  -------
 *  RefCell    borrow, borrow_mut (dynamic borrowing)
 *  -------
 *  Listnode   .next field accessor
 *
 *
 * unwrap()         Some
 *                   ^
 * |                 |
 * |-------clone-----|
 */


pub type ListSNodeRef = Rc<RefCell<ListSNode>>;

#[derive(PartialEq, Eq, Debug)]
pub struct ListSNode {
    pub data: i32,
    pub next: Option<ListSNodeRef>
}

impl ListSNode {
    pub fn new(data: i32) -> Option<ListSNodeRef> {
        Some(
            Rc::new(
                RefCell::new(
                    ListSNode {
                        next: None,
                        data
                    }
                )
            )
        )
    }

    pub fn clone(node: &Option<ListSNodeRef>) -> Option<ListSNodeRef> {
        node.clone() //node.as_ref().map(Rc::clone)
    }

    pub fn next(current: &Option<ListSNodeRef>) -> Option<ListSNodeRef> {
        if current.is_none() { return None }
        //current.as_ref().unwrap().borrow().next.as_ref().map(Rc::clone)
        current.as_ref().unwrap().borrow().next.clone()
    }

    pub fn set_next(current: &Option<ListSNodeRef>, next: Option<ListSNodeRef>) {
        if current.is_none() { return }
        current.as_ref().unwrap().borrow_mut().next = next;
    }

    pub fn from_list(a: &[i32]) -> Option<ListSNodeRef> {
        let mut head: Option<ListSNodeRef> = None;
        let mut n;

        // Reverse the array list so 4 then points to 5 etc..
        for v in a.iter().rev() {
            n = ListSNode::new(*v);
            n.as_ref().unwrap().borrow_mut().next = head.clone(); // Some(Rc::clone(&head.unwrap()));
            head = n;
        }

        head
    }
}

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub terminal: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            terminal: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        self.add_word(word);
    }

    pub fn add_word(&mut self, word: String) {
        let mut node = self;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(|| TrieNode::new())
        }

        node.terminal = true;
    }

    pub fn search_helper(&self, index: usize, len: usize, word: &[char]) -> bool {
        let mut current = self;
        let mut ch;

        for i in index..len {
            ch = word.get(i).unwrap();

            if let Some(tmp) = current.children.get(&ch) {
                current = tmp;
            } else if *ch == '.' {
                for v in current.children.values() {
                    if v.search_helper(i+1, len, word) { return true };
                }
            } else {
                return false
            }
        }

        if current.terminal { true } else { false } 
    }

    pub fn search(&self, word: String) -> bool {
        let list: Vec<char> = word.chars().collect();
        self.search_helper(0, list.len(), &list)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;

        for ch in prefix.chars() {
            if let Some(tmp) = current.children.get(&ch) {
                 current = tmp;
            } else {
                 return false
            }
        }

        true
    }
}
