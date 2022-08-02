/*
 * 178 · Graph Valid Tree

Given n nodes labeled from 0 to n - 1 and a list of undirected edges (each edge is a pair of nodes), write a function to check whether these edges make up a valid tree.

You can assume that no duplicate edges will appear in edges. Since all edges are undirected, [0, 1] is the same as [1, 0] and thus will not appear together in edges.
Example

Example 1:

Input: n = 5 edges = [[0, 1], [0, 2], [0, 3], [1, 4]]

Output: true.

Example 2:

Input: n = 5 edges = [[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]]

Output: false.
 */

use std::collections::{HashSet, HashMap, VecDeque};

pub struct Solution {}

impl Solution {
    // @param n: An integer
    // @param edges: a list of undirected edges
    // @return: true if it's a valid tree, or false

    // A graph that is a valid tree does not contain cycles
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        // simulate a tree with a HashMap populating the edges info

        // Node value is K, Vec<i32> of neighbors list is V
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for pair in edges.iter() {
            if 2 == pair.len() {
                // If pair is [x, y], store x in hashmap storing y as its neighbor and alternately
                //                    store y in hashmap storing x as its neighbor
                // map
                // x -> [y, ..]
                // y -> [x, ..]

                // or for test case 1
                // 0 -> [1, 2, 3]
                // 1 -> [0, 4]
                // 2 -> [0]
                // 3 -> [0]
                // 4 -> [1]

                map.entry(pair[0]).or_insert_with(|| vec![]).push(pair[1]);
                map.entry(pair[1]).or_insert_with(|| vec![]).push(pair[0]);
            }
        }

        let mut visited: HashSet<i32> = HashSet::new();
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();

        let seed = (0, -1); // start with node 0, and its prev -1
        q.push_back(seed);

        while !q.is_empty() {
            let (current, previous) = q.pop_front().unwrap();

            // means a cycle is present, as current has already been visited
            if visited.contains(&current) {
                return false
            }

            visited.insert(current);

            // enqueue all of current's adjacent neighbors
            for next in map.get(&current).unwrap().iter() {

                // safely ignore the neighbor that was just departed
                // (no need to redo work and to false trigger already visited early return)
                if *next == previous {
                    continue
                }

                // push back (neighbor, neigbhor's prev)
                q.push_back((*next, current));
            }
        }

        n == visited.len() as i32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0261() {
        assert_eq!(true, Solution::valid_tree(5,vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]]));
        assert_eq!(false, Solution::valid_tree(5, vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]]));
    }
}

