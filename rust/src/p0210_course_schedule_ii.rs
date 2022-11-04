/*
210. Course Schedule II
Medium

There are a total of n courses you have to take labelled from 0 to n - 1.

Some courses may have prerequisites, for example, if prerequisites[i] = [ai, bi] this means you must take the course bi before the course ai.

Given the total number of courses numCourses and a list of the prerequisite pairs, return the ordering of courses you should take to finish all courses.

If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.

Example 1:

Input: numCourses = 2, prerequisites = [[1,0]]
Output: [0,1]
Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].

Example 2:

Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
Output: [0,2,1,3]
Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].

Example 3:

Input: numCourses = 1, prerequisites = []
Output: [0]

Constraints:

    1 <= numCourses <= 2000
    0 <= prerequisites.length <= numCourses * (numCourses - 1)
    prerequisites[i].length == 2
    0 <= ai, bi < numCourses
    ai != bi
    All the pairs [ai, bi] are distinct.
 */

use std::collections::VecDeque;

// Easier to have a tag to use when indexing otherwise this becomes an unncecessary source of confusion
const SRC: usize = 1;
const DST: usize = 0;

pub struct Solution {}

impl Solution {

    // n is the number of courses or vertices
    // prerequisites are a well-formed list of edges
    pub fn run(n: u16, edges: &Vec<[u16; 2]>) -> Vec<u16> {
        let mut result = Vec::new();
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
        for i in 0..in_degrees.len() {
            if in_degrees[i] == 0 {
                queue.push_back(i as u16);
            }
        }

        // process the in degrees of the vertices
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap(); // remove from the queue and update indegree count(s)
            result.push(current); // every time we dequeue, add it to our results list (as it now has no prereqs or dependencies anymore)
            count += 1;

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

        if n == count { return result } else { return vec![0] };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0210() {
        // Note vertices need to be 0..n
        assert_eq!(vec![0,1], Solution::run(2, &vec![[1,0]]));
        assert_eq!(vec![0,1,2,3], Solution::run(4, &vec![[1,0],[2,0],[3,1],[3,2]]));
        assert_eq!(vec![0], Solution::run(1, &vec![]));
        // 4 -> 3, 3 -> 1, 1 -> 0, 0 -> 2
        assert_eq!(vec![4,3,1,0,2], Solution::run(5, &vec![[3,4], [1,3], [2,0], [0,1]]));
    }
}
