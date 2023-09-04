/*
 * 133. Clone Graph
Medium

Given a reference of a node in a connected undirected graph.

Return a deep copy (clone) of the graph.

Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.

class Node {
    public int val;
    public List<Node> neighbors;
}

 

Test case format:

For simplicity, each node's value is the same as the node's index (1-indexed). For example, the first node with val == 1, the second node with val == 2, and so on. The graph is represented in the test case using an adjacency list.

An adjacency list is a collection of unordered lists used to represent a finite graph. Each list describes the set of neighbors of a node in the graph.

The given node will always be the first node with val = 1. You must return the copy of the given node as a reference to the cloned graph.
 *
 *
 */

use std::collections::HashMap;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

/*** Solution 1 ***/

#[derive(Debug, PartialEq, Eq)]
pub struct NodeRef(Rc<RefCell<Node>>); // add new type wrapper around Rc'ed Node


#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub value: i32,
    pub neighbors: Vec<NodeRef>
}

impl std::fmt::Display for NodeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut slist = String::new();

        for n in self.0.borrow().neighbors.iter() {
//        for n in self.neighbors() {
            slist.push_str(&n.value().to_string());
            slist.push_str(", ");
        }

        write!(f, "(value: {}, neighbors: {})", self.value(), slist)
    }
}

impl Node {
    pub fn new(value: i32) -> NodeRef {
        NodeRef(
            Rc::new(
                RefCell::new(
                    Node {
                        value,
                        neighbors: vec![],
                    }
                )
            )
        )
    }

    pub fn from_list(input: &Vec<Vec<i32>>) -> NodeRef {
        let mut map = HashMap::new();

        for i in 1..=input.len() { // start at 1 index
            map.insert(i as i32, Node::new(i as i32));
        }

        for (i, subvec) in input.iter().enumerate() {
            let n = map.get(&(i as i32 + 1)).unwrap(); // add 1 since starts at 0

            for peer_id in subvec.iter() {
                let peer = map.get(peer_id).unwrap();
                n.add_neighbor(peer);
            }
        }

        map.remove(&1).unwrap() // return the first node
    }

    pub fn to_list(adjmap: &HashMap<i32, NodeRef>) -> Vec<Vec<i32>> {
        use std::collections::BTreeMap;

        let mut bmap = BTreeMap::new(); // easier to do asserts since btreemap is sorted

        for (k,v) in adjmap.iter() {
            bmap.insert(k, v.peers());
        }

        bmap.into_values().collect()
    }
}

// abstract away most of the Rc and RefCell specific methods through the newtype
impl NodeRef {
    pub fn value(&self) -> i32 {
        self.0.borrow().value // i32 has copy semantics
    }

    pub fn clone(&self) -> NodeRef {
        NodeRef(Rc::clone(&self.0))
    }

    pub fn add_neighbor(&self, entry: &NodeRef) {
        self.0.borrow_mut().neighbors.push(entry.clone())
    }
/*
    pub fn neighbors(&self) -> std::slice::Iter<'_, NodeRef> {
        self.0.borrow().neighbors.iter()
    }
*/

    pub fn peers(&self) -> Vec<i32> {
        self.0.borrow().neighbors.iter().map(|entry| entry.value()).collect::<Vec<i32>>()
    }
}

pub struct Solution1 {}

impl Solution1 {
    pub fn clone_graph(node: NodeRef) -> HashMap<i32, NodeRef> { // }NodeRef {
        let mut cloned_map: HashMap<i32, NodeRef> = HashMap::new();
        let _cloned = Solution1::dfs(node, &mut cloned_map);

//        println!("cloned is {}", cloned);
        
        cloned_map
    }

    pub fn dfs(original: NodeRef, cloned_map: &mut HashMap<i32, NodeRef>) -> NodeRef {
        let key = original.value();

        if let Some(v) = cloned_map.get(&key) {
            return v.clone()
        }

        let cloned = Node::new(key);
        cloned_map.insert(key, cloned.clone());

        for peer in original.0.borrow().neighbors.iter() {
//        for peer in original.neighbors() {
            let cloned_peer = Solution1::dfs(peer.clone(), cloned_map);
            cloned.add_neighbor(&cloned_peer);
        }

        cloned
    }
}





/*** Solution 2 ***/

#[derive(Debug, PartialEq, Eq)]
pub struct GraphNode {
    pub label: i32,
    pub original: bool,
    pub neighbors: Vec<Rc<RefCell<GraphNode>>>
}

impl std::fmt::Display for GraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        let mut list = String::new();

        for x in self.neighbors.iter() {
            list.push_str(&x.borrow().label.to_string());
            list.push_str(", ");
        }

        write!(f, "(label: {}, orig: {}, neighbors: {})", self.label, self.original, list)
    }
}

impl GraphNode {
    pub fn new(label: i32, original: bool) -> Rc<RefCell<GraphNode>> {
        Rc::new(
           RefCell::new(
               GraphNode {
                   label,
                   original,
                   neighbors: vec![],
               }
           )
        )
    }

    pub fn add(&mut self, entry: &Rc<RefCell<GraphNode>>) {
        self.neighbors.push(Rc::clone(entry))
    }

    pub fn neighbors(&self) -> std::slice::Iter<'_, Rc<RefCell<GraphNode>>> {
        self.neighbors.iter()
    }

    pub fn peers(&self) -> Vec<i32> {
        self.neighbors.iter().map(|entry| entry.borrow().label).collect::<Vec<i32>>()
    }

    pub fn to_list(map: &HashMap<i32, Rc<RefCell<GraphNode>>>) -> Vec<Vec<i32>>{
        use std::collections::BTreeMap;

        /* Essentially map is:
        map 4 is (label: 4, orig: false, neighbors: 1, 3, )
        map 3 is (label: 3, orig: false, neighbors: 2, 4, )
        map 1 is (label: 1, orig: false, neighbors: 2, 4, )
        map 2 is (label: 2, orig: false, neighbors: 1, 3, )
        */

        let mut bmap = BTreeMap::new();

        for (k,v) in map.iter() {
            if v.borrow().original == false { // ensure not original
                bmap.insert(k, v.borrow().peers());
            }
        }

        bmap.into_values().collect()
    }

    pub fn from_list(input: &Vec<Vec<i32>>) -> Rc<RefCell<GraphNode>> {
        let mut map = HashMap::new();

        for i in 1..=input.len() { // start at 1 index
            let node = GraphNode::new(i as i32, true); // mark as original
            map.insert(i as i32, Rc::clone(&node));
        }

        for (i, subvec) in input.iter().enumerate() {
            let entry = map.get(&(i as i32 + 1)).unwrap(); // add 1 since starts at 0

            for peer_id in subvec.iter() {
                let peer = map.get(peer_id).unwrap();
                entry.borrow_mut().add(peer);
            }
        }

        /*

        This procedure replicates these instructions

        let node1 = GraphNode::new(1, true); // 2,4
        let node2 = GraphNode::new(2, true); // 1,3
        let node3 = GraphNode::new(3, true); // 2,4
        let node4 = GraphNode::new(4, true); // 1,3

        node1.borrow_mut().add(&node2);
        node1.borrow_mut().add(&node4);

        node2.borrow_mut().add(&node1);
        node2.borrow_mut().add(&node3);

        node3.borrow_mut().add(&node2);
        node3.borrow_mut().add(&node4);

        node4.borrow_mut().add(&node1);
        node4.borrow_mut().add(&node3);
         */

        map.remove(&1).unwrap() // return the first node
    }
}

pub struct Solution2 {}

impl Solution2 {
    pub fn clone_graph(node: &Rc<RefCell<GraphNode>>) -> HashMap<i32, Rc<RefCell<GraphNode>>> {
        // BFS style iteration with a hashmap to track visited nodes, as well as to add shared neighbors

        let mut key: i32 = node.borrow().label;

        let mut q: VecDeque<Rc<RefCell<GraphNode>>> = VecDeque::new();
        let mut map: HashMap<i32, Rc<RefCell<GraphNode>>> = HashMap::new();


        // seed queue and map with initial node and node's clone respectively
        q.push_back(Rc::clone(node));
        map.insert(key, GraphNode::new(key, false));

        // while queue is not empty, pop from it and add its peers/children
        while !q.is_empty() {
            let mut aux_map: HashMap<i32, Rc<RefCell<GraphNode>>> = HashMap::new();

            let current = q.pop_front().unwrap();
            let cloned = map.get(&current.borrow().label).unwrap();
            let mut peer;

            // Iterate through unvisited current node's neighbors
            for n in current.borrow().neighbors() {
                // if node n hasn't been visited yet, add it to the queue
                // add it to the map with it's clone

                key = n.borrow().label;

                if !map.contains_key(&key) {
                    q.push_back(Rc::clone(n));
                    peer = GraphNode::new(key, false);
                    aux_map.insert(key, Rc::clone(&peer));
                    cloned.borrow_mut().add(&peer);
                } else {
                    // add new cloned neighbors
                    cloned.borrow_mut().add(map.get(&key).unwrap());
                }
            }

            map.extend(aux_map);
        }

        map
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0133() {
        // test 1
        let adjlist = vec![vec![2,4], vec![1,3], vec![2,4], vec![1,3]];
        let node1 = GraphNode::from_list(&adjlist);
        let map = Solution2::clone_graph(&node1);
        let result = GraphNode::to_list(&map);

        assert_eq!(vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]], result);

        // test 1
        let adjlist = vec![vec![2,4], vec![1,3], vec![2,4], vec![1,3]];
        let node1 = Node::from_list(&adjlist);
        let map = Solution1::clone_graph(node1.clone());
        let result = Node::to_list(&map);

        assert_eq!(vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]], result);


        // test 2
        let adjlist: Vec<Vec<i32>> = vec![vec![]];
        let node1 = GraphNode::from_list(&adjlist);
        let map = Solution2::clone_graph(&node1);
        let result = GraphNode::to_list(&map);

        assert_eq!(adjlist, result);

        // test 3
        // Need to wrap return values in Option to handle empty graph 
    }
}

/*
test 1

top node is (label: 1, orig: false, neighbors: 2, 4, )
bottom node is (label: 2, orig: false, neighbors: 1, 3, ) its neighbors are (label: 1, orig: false, neighbors: 2, 4, )
bottom node is (label: 2, orig: false, neighbors: 1, 3, ) its neighbors are (label: 3, orig: false, neighbors: 2, 4, )
bottom node is (label: 4, orig: false, neighbors: 1, 3, ) its neighbors are (label: 1, orig: false, neighbors: 2, 4, )
bottom node is (label: 4, orig: false, neighbors: 1, 3, ) its neighbors are (label: 3, orig: false, neighbors: 2, 4, )

1 -> 2,4   2 -> 1,3   3 -> 2,4   4 -> 1,3

*/
