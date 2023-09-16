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
        node.as_ref().unwrap().borrow().data
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


/* Box List Node */

pub type ListNodeRef = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub data: i32,
    pub next: ListNodeRef
}

impl ListNode {
    pub fn new(data: i32) -> ListNodeRef {
        Some(Box::new(ListNode {
            next: None,
            data
        }))
    }

    pub fn to_list(a: &[u32]) -> ListNodeRef {
        let mut head: ListNodeRef = None;

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


pub type ListSNodeRef = Option<Rc<RefCell<ListSNode>>>;

#[derive(PartialEq, Eq, Debug)]
pub struct ListSNode {
    pub data: i32,
    pub next: ListSNodeRef
}

impl ListSNode {
    pub fn new(data: i32) -> ListSNodeRef {
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

    pub fn clone(node: &ListSNodeRef) -> ListSNodeRef {
        node.clone() //node.as_ref().map(Rc::clone)
    }

    pub fn next(current: &ListSNodeRef) -> ListSNodeRef {
        if current.is_none() { return None }
        //current.as_ref().unwrap().borrow().next.as_ref().map(Rc::clone)
        current.as_ref().unwrap().borrow().next.clone()
    }

    pub fn set_next(current: &ListSNodeRef, next: ListSNodeRef) {
        if current.is_none() { return }
        current.as_ref().unwrap().borrow_mut().next = next;
    }

    pub fn from_list(a: &[i32]) -> ListSNodeRef {
        let mut head: ListSNodeRef = None;
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
