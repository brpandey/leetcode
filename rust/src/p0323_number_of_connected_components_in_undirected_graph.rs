/*
 * 323. Number of Connected Components in an Undirected Graph
 * Problem:

Given n nodes labeled from 0 to n - 1 and a list of undirected edges (each edge is a pair of nodes), write a function to find the number of connected components in an undirected graph.

Example 1:
     0          3
     |          |
     1 --- 2    4
Given n = 5 and edges = [[0, 1], [1, 2], [3, 4]], return 2.

Example 2:
     0           4
     |           |
     1 --- 2 --- 3
Given n = 5 and edges = [[0, 1], [1, 2], [2, 3], [3, 4]], return 1.

Note:
You can assume that no duplicate edges will appear in edges. Since all edges are undirected, [0, 1] is the same as [1, 0] and thus will not appear together in edges.

 *
 */



/*
  Strategy is to determine each node's parent.
  Parent's are arbitrarily defined, but the "arbritary scheme" is consistent
  First, each node is deemed a parent of itself (this is before looking at the edges data)

  Using test case 1 to illustrate

  Indices indicate node number, values indicate parent node number
   0  1  2  3  4
  [0][1][2][3][4]

  Second, edge info is incorporated, and the second edge point of an edge is the parent, say [0,1] is the edge,
  1 is the second edge point of the edge, so 1 is (arbitrarily) made parent of 0, and 1 is already parent of 1.

  0 could have arbitrarily been chosen as the parent of 1, but it doesn't matter, as long as we stick to one scheme

  Third, the parent data is reduced or compressed, so that we don't have any intermediate parents, just the root parents
  So if the parent's list is

   0  1  2  3  4               0  1  2  3  4
  [1][2][2][4][4] it becomes  [2][2][2][4][4]


  Lastly, store parent data in HashSet to eliminate duplicates and take size
*/

use std::collections::{HashSet, HashMap};

pub struct Solution1 {}

// Union Find approach
impl Solution1 {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // create a vector with each node value signifying they're a parent to themself
        // essentially each item item is its own subset
        let mut parents: Vec<i32> = (0..n).collect(); 

        // process edges data into parents
        // essentially merge subsets together
        for e in edges.iter() {
            if e.len() == 2 {
                Solution1::union(e[0], e[1], &mut parents) // incorporate edge info
            }
        }

        for p in 0..n {
              Solution1::find(p, &mut parents);
        }

        // return the count of unique parents - or connected components
        parents.into_iter().collect::<HashSet<i32>>().len() as i32
    }

    // Each of the two nodes since they are connected by an edge should be in the same subset
    // by making one subset's parent point to the other subset's parent
    pub fn union(node1: i32, node2: i32, parents: &mut Vec<i32>) {
        // Merge subsets
        // Refer to Second explanation above of [0,1], so 1 is made parent of 0
        let p1 = Solution1::find(node1, parents);
        let p2 = Solution1::find(node2, parents);
        parents[p1 as usize] = p2;
    }


    // Put the root parent into that node's parent list value, by following the parent chain
    pub fn find(node: i32, parents: &mut Vec<i32>) -> i32 {
        // assumption, parent of subset is when the parent of index = index's value
        // this is a form of path compression as when done 0 now points to 2 as its parent
        // instead of an intermediate 1, and then 1's parent as 2
        if node != parents[node as usize] { // not subset parent yet so keep going
            parents[node as usize] = Solution1::find(parents[node as usize], parents);
        }

        parents[node as usize]
    }
}

// Using traditional dfs
pub struct Solution2 {}

impl Solution2 {
    // reorganize list of edge pairs into map of node to its edges (adjacency list)
    pub fn setup(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>>{
        let mut map = HashMap::new();

        for e in edges.iter() {
            if e.len() == 2 {
                map.entry(e[0]).or_insert_with(|| vec![]).push(e[1]);
                map.entry(e[1]).or_insert_with(|| vec![]).push(e[0]);
            }
        }

//        dbg!(&map);

        map
    }

    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let map = Solution2::setup(edges);
        let mut visited: Vec<bool> = vec![false; n as usize];
        let mut count: i32 = 0;

        // Loop through the number of unique nodes, for each node not visited yet,
        // do a dfs to mark all the reachable nodes from this current node as visited
        for i in 0..n as usize{
            if !visited[i] {
                count += 1;
                Solution2::dfs(i as i32, &map, &mut visited);
            }
        }

        count
    }

    pub fn dfs(node: i32, map: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>) {
        if let Some(peers) = map.get(&node) {
            visited[node as usize] = true;

            // do recursive dfs on peer node if it hasn't been visited
            for peer_node in peers.iter() {
                if false == visited[*peer_node as usize] {
                    Solution2::dfs(*peer_node, map, visited)
                }
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0323() {
        assert_eq!(2, Solution1::count_components(5, vec![vec![0,1], vec![1,2], vec![3,4]]));
        assert_eq!(1, Solution1::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]));

        assert_eq!(2, Solution2::count_components(5, vec![vec![0,1], vec![1,2], vec![3,4]]));
        assert_eq!(1, Solution2::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]));

    }
}

/*
First test case
find success: node is 0, parents[0] is 0, parents is [0, 1, 2, 3, 4]
find success: node is 1, parents[1] is 1, parents is [0, 1, 2, 3, 4]
union: parent of node 0 is now 1
find success: node is 1, parents[1] is 1, parents is [1, 1, 2, 3, 4]
find success: node is 2, parents[2] is 2, parents is [1, 1, 2, 3, 4]
union: parent of node 1 is now 2
find success: node is 3, parents[3] is 3, parents is [1, 2, 2, 3, 4]
find success: node is 4, parents[4] is 4, parents is [1, 2, 2, 3, 4]
union: parent of node 3 is now 4
Done with union phase

find node is 0, parents[0] is 1, parents is [1, 2, 2, 4, 4]
find node is 1, parents[1] is 2, parents is [1, 2, 2, 4, 4]
find success: node is 2, parents[2] is 2, parents is [1, 2, 2, 4, 4]
find success: node is 1, parents[1] is 2, parents is [1, 2, 2, 4, 4]
find success: node is 0, parents[0] is 2, parents is [2, 2, 2, 4, 4]
find node is 1, parents[1] is 2, parents is [2, 2, 2, 4, 4]
find success: node is 2, parents[2] is 2, parents is [2, 2, 2, 4, 4]
find success: node is 1, parents[1] is 2, parents is [2, 2, 2, 4, 4]
find success: node is 2, parents[2] is 2, parents is [2, 2, 2, 4, 4]
find node is 3, parents[3] is 4, parents is [2, 2, 2, 4, 4]
find success: node is 4, parents[4] is 4, parents is [2, 2, 2, 4, 4]
find success: node is 3, parents[3] is 4, parents is [2, 2, 2, 4, 4]
find success: node is 4, parents[4] is 4, parents is [2, 2, 2, 4, 4]
*/
