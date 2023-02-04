/*

743. Network Delay Time

You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.

We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.

Example 1:

Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
Output: 2

Example 2:

Input: times = [[1,2,1]], n = 2, k = 1
Output: 1

Example 3:

Input: times = [[1,2,1]], n = 2, k = 2
Output: -1

*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, HashMap};

pub struct Solution {}

// Note djikstra's algorithm tracks (in a table) these
// fields: vertex, shortest distance from A (start vertex), and previous vortex
// Since, an actual shortest path algorithm does not need to be reproduced, just the shortest
// time, tracking the previous vortex isn't required
// Hence, just need to track shortest distance from A (start vertex) e.g. path and vertex
// Also, don't need to update recorded distances just to push onto min heap
// Min heap reflects our search space

/*
  Djikstra's algorithm pseudo code

  Let distance of start vertex from start vertex (e.g. A) = 0
  Let distance of all other vertices from start = infinity

  While vertices remain unvisited
    Visit unvisited vertex with smallest known distance from start vertex (current vertex) -- greedy step
    For each unvisited neighbor of current vertex
      Calculate distance from start vertex
        If the calculated distance of current vertex is less than recorded distance
          Update shortest distance to vertex

    Add current vertex to list of visited vertices
*/

// Use djikstra's algorithm (greedy - pick node's shortest weight neighbor vertex to evaluate first)
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        //convert list of edges into an adjacency graph, for ease to grab neighbors
        let map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        let adjlist = times.into_iter().fold(map, |mut acc, edge| {
            let (s, d, t) = (edge[0], edge[1], edge[2]);
            acc.entry(s).and_modify(|v| v.push((d,t))).or_insert(vec![(d,t)]);
            acc
        });

        // acc is {2: [(1, 1), (3, 1)], 3: [(4, 1)]}
        // println!("acc is {:?}", &adjlist);

        let mut acc_t = 0;
        let mut visited: HashSet<i32> = HashSet::new();
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new(); // entry is (time, vertex)

        // seed min heap with start vertex k
        min_heap.push(Reverse((0, k))); // the time value is first in the tuple so heap can compare properly

        while !min_heap.is_empty() {
            // time path current, vertex current
            let Reverse((t, v)) = min_heap.pop().unwrap(); // this is greedy part, look at smallest time path node

            if visited.contains(&v) { // if neighbor v has been visited
                continue
            }

            if let Some(current_neighbors) = adjlist.get(&v) {
                for (vn, tn) in current_neighbors { // vertex n, time n -- do bfs of neighbors
                    if !visited.contains(&vn) {
                        min_heap.push(Reverse((t + tn, *vn))); // only push neighbor if unvisited
                    }
                }
            }

            visited.insert(v); // mark visited since vertex has been processed
            acc_t = std::cmp::max(acc_t, t); // updated acc_t as min 
        }

        // if there was a path to all nodes, visited should be the same as n
        if visited.len() == n as usize {
            return acc_t
        } else {
            -1
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0743() {
        assert_eq!(2, Solution::network_delay_time(vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]], 4, 2));
        assert_eq!(1, Solution::network_delay_time(vec![vec![1,2,1]], 2, 1));
        assert_eq!(-1, Solution::network_delay_time(vec![vec![1,2,1]], 2, 2));
    }
}
