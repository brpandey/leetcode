/*
 * 146. LRU Cache
Medium

Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

Implement the LRUCache class:

    LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
    int get(int key) Return the value of the key if the key exists, otherwise return -1.
    void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.

The functions get and put must each run in O(1) average time complexity.

Example 1:

Input
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
Output
[null, null, null, 1, null, -1, null, -1, 3, 4]
 *
 */

/* Strategy
 * lrucache contains a hashmap as well as the head (left) and tail (right) 
 * of a doubly linked list
 * head is a dummy head that points to (via next) the least recently used node
 * tail is a dummy tail that points to (via prev) the most recently used node
 * On every access (e.g.) get, return node's value, and remove then reinsert node
 * so that it is at the end of the list closest to the most recently used marker
 *
 * if hashmap is at capacity evict a node, the one pointed to (via next) from the least
 * recently used node marker or head or left 
 */

//use crate::util::DLNode;
//use crate::util::DLNodeRef;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub type DLNodeRef<T> = Rc<RefCell<DLNode<T>>>;

#[derive(PartialEq, Eq, Debug)]
pub struct DLNode<T> { // Doubly Linked Node
    pub key: T,
    pub data: T,
    pub next: Option<DLNodeRef<T>>,
    pub prev: Option<DLNodeRef<T>>,
}

impl<T> DLNode<T> {
    pub fn new(key: T, data: T) -> DLNodeRef<T> {
        Rc::new(
            RefCell::new(
                DLNode {
                    next: None,
                    prev: None,
                    data,
                    key
                }
            )
        )
    }

    pub fn clone(node: &Option<DLNodeRef<T>>) -> Option<DLNodeRef<T>> {
        node.as_ref().map(Rc::clone)
    }

    pub fn next(current: &Option<DLNodeRef<T>>) -> Option<DLNodeRef<T>> {
        if current.is_none() { return None }
        current.as_ref().unwrap().borrow().next.as_ref().map(Rc::clone)
    }

    pub fn prev(current: &Option<DLNodeRef<T>>) -> Option<DLNodeRef<T>> {
        if current.is_none() { return None }
        current.as_ref().unwrap().borrow().prev.as_ref().map(Rc::clone)
    }

    pub fn remove(current: &Option<DLNodeRef<T>>) {
        // remove 
        // 1) takes the current prev Node and sets its next to the current next Node
        // 2) takes the current next Node and sets its prev to the current prev Node
        // 3) sets current prev to None and current next to None
        let prev = DLNode::prev(current);
        let next = DLNode::next(current);

        // take care of incoming links to current
        prev.as_ref().unwrap().borrow_mut().next = DLNode::clone(&next);
        next.as_ref().unwrap().borrow_mut().prev = DLNode::clone(&prev);

        // take care of outgoing links from current
        current.as_ref().unwrap().borrow_mut().next = None;
        current.as_ref().unwrap().borrow_mut().prev = None;
    }

    pub fn insert(current: &Option<DLNodeRef<T>>, tail: &Option<DLNodeRef<T>>) {
        // insert right before dummy tail node
        // dummy tail signifies node AFTER 
        // most recently used node (e.g. just did a put on this node or get)

        // 1) takes tail prev Node and sets its next to the current Node
        // 2) sets current prev Node to tail prev node
        // 3) takes tail prev and sets it to current Node
        // 4) sets current next Node to tail node

        let prev = DLNode::prev(tail);

        // link current and prev
        prev.as_ref().unwrap().borrow_mut().next = DLNode::clone(current);
        current.as_ref().unwrap().borrow_mut().prev = DLNode::clone(&prev);
        
        // link current and tail
        tail.as_ref().unwrap().borrow_mut().prev = DLNode::clone(current);
        current.as_ref().unwrap().borrow_mut().next = DLNode::clone(tail);
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct LRUCache {
    pub map: HashMap<i32, DLNodeRef<i32>>,
    pub left: Option<DLNodeRef<i32>>, // least recently used
    pub right: Option<DLNodeRef<i32>>, // most recently used
    pub capacity: usize,
}


impl LRUCache {

    pub fn new(capacity: usize) -> Self {
        let cache = LRUCache {
            map: HashMap::new(),
            left: Some(DLNode::new(-1,-1)),
            right: Some(DLNode::new(-1,-1)),
            capacity,
        };

        // Link left with right, left <-> right 
        cache.left.as_ref().unwrap().borrow_mut().next = DLNode::clone(&cache.right);
        cache.right.as_ref().unwrap().borrow_mut().prev = DLNode::clone(&cache.left);
    
        cache
    }
    
    pub fn get(&self, key: i32) -> i32 {
        if let Some(val) = self.map.get(&key) {
            let value = Some(Rc::clone(val));
            // remove node from current doubly linked list setup
            DLNode::remove(&value);
            // re-insert next to most recently used
            DLNode::insert(&value, &self.right);
            let result = value.as_ref().unwrap().borrow().data;
            result
        } else {
            -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        // at peak capacity, evict one to create space for the put
        if self.map.len() == self.capacity {
            // evict least recently used
            let evict = DLNode::next(&self.left);
            DLNode::remove(&evict);
            self.map.remove(&evict.as_ref().unwrap().borrow().key);
        }

        // create new node, insert next to most recently used marker
        // put in map
        let node = Some(DLNode::new(key, value));
        DLNode::insert(&node, &self.right);
        self.map.insert(key, node.unwrap());
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0146() {
        let mut l = LRUCache::new(2);
        l.put(1,1);
        l.put(2, 2); // cache is {1=1, 2=2}
        assert_eq!(1, l.get(1));    // return 1
        l.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        assert_eq!(-1, l.get(2));    // returns -1 (not found)
        l.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        assert_eq!(-1, l.get(1));    // return -1 (not found)
        assert_eq!(3, l.get(3));    // return 3
        assert_eq!(4, l.get(4));    // return 4
    }
}

