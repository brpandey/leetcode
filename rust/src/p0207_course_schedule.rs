/*
207. Course Schedule
Medium

There are a total of numCourses courses you have to take, labeled from 0 to numCourses-1.

Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]

Given the total number of courses and a list of prerequisite pairs, is it possible for you to finish all courses?

Example 1:

Input: numCourses = 2, prerequisites = [[1,0]]
Output: true
Explanation: There are a total of 2 courses to take. 
             To take course 1 you should have finished course 0. So it is possible.

Example 2:

Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false
Explanation: There are a total of 2 courses to take. 
             To take course 1 you should have finished course 0, and to take course 0 you should
             also have finished course 1. So it is impossible.

Constraints:

    The input prerequisites is a graph represented by a list of edges, not adjacency matrices. Read more about how a graph is represented.
    You may assume that there are no duplicate edges in the input prerequisites.
    1 <= numCourses <= 10^5


 */

use std::collections::VecDeque;

// Easier to have a tag to use when indexing otherwise this becomes an unncecessary source of confusion
const SRC: usize = 1;
const DST: usize = 0;

pub struct Solution {}

impl Solution {

    // n is the number of courses or vertices
    // prerequisites are a well-formed list of edges
    pub fn run(n: u16, edges: &Vec<[u16; 2]>) -> bool {
        let mut count: u16 = 0;
        let mut in_degrees = vec![0; n as usize];

        // Set the in degrees of the vertices
        // NOTE: If prerequisites is [[1,0]], the graph looks like: 0 -> 1 or SRC -> DEST or [[DEST, SRC]]
        // Essentially the second or dst element of the edge is what the edge is pointing to so increment
        // that vertex's indegree count
        for e in edges {
            in_degrees[e[DST] as usize] += 1;
        }

        let mut queue: VecDeque<u16> = VecDeque::new();

        // Find all the vertices which have an in degree of 0 (meaning no dependencies or back arrows) and seed the queue
        for &i in &in_degrees {
            if i == 0 {
                queue.push_back(i as u16);
            }
        }

        // process the in degrees of the vertices
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap(); // remove from the queue and update indegree count(s)
            count += 1; // every time we dequeue, we increment count (as we know it has no prereqs or dependencies)

            // loop through the "graph" again search for if the vertex with nothing pointing at it has directed edges towards
            // destination vertices (atleast 1 must exist)
            for e in edges {
                // if current is equal to the edge src vertex, and since we've removed current from the queue already,
                // decrement the edge dest's vertex indegree, since we no longer have a vertex pointing towards it
                if current == e[SRC] {
                    in_degrees[e[DST] as usize] -= 1;
                    // if the edge destination vertex has no vertices pointing to it, add it to the queue
                    if in_degrees[e[DST] as usize] == 0 {
                        queue.push_back(e[DST]);
                    }
                }
            }
        }

        n == count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0207() {
        assert_eq!(true, Solution::run(2, &vec![[1,0]]));
        // 0 <---> 1, essentially we have a cycle with indegree never becoming 0 and never getting on queue in the first place hence count = 0
        assert_eq!(false, Solution::run(2, &vec![[1,0], [0,1]]));
    }
}
