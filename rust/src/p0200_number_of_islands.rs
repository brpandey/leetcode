/*
200. Number of Islands
Medium

Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.


Example 1:

Input: grid = [
["1","1","1","1","0"],
["1","1","0","1","0"],
["1","1","0","0","0"],
["0","0","0","0","0"]
]
Output: 1

Example 2:

Input: grid = [
  ["1","1","0","0","0"],
  ["1","1","0","0","0"],
  ["0","0","1","0","0"],
  ["0","0","0","1","1"]
]
Output: 3

 */

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn run(matrix: &Vec<Vec<char>>) -> u8 {

        let mut islands_count = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();

        // Ensure we don't duplicate our calculations by revisiting a cell we've already been to
        // e.g. when '1' is below another '1', we don't want a cycle going back and forth between the two
        let visited: &mut Vec<Vec<bool>> = &mut vec![vec![false; cols]; rows];

        // Generate once the additive sequence offsets that we want to visit per (r,c) location
        let sequence = Solution::offsets();

        for r in 0..rows {
            for c in 0..cols {
                // Check current cell
                if matrix[r][c] == '1' && visited[r][c] == false {
                    // We've found a place to explore that identifies a new connected component cluster
                    Solution::bfs(matrix, visited, &sequence, (r as i32, c as i32), (rows as i32, cols as i32));
                    islands_count += 1;
                }
            }
        }

        islands_count
    }

    // Offsets generator
    pub fn offsets() -> Vec<(i32,i32)> {
        // Visit the four peers (adjacent horizontal and vertical)
        let cycle: [i32; 4] = [0, -1, 0, 1];
        let rows: Vec<i32> = cycle.iter().cycle().take(4).cloned().collect(); // who are 0, -1, 0, 1 rows away
        let cols: Vec<i32> = cycle.iter().cycle().skip(1).take(4).cloned().collect(); // -1, 0, 1, 0, shift by one to get cols
        let sequence: Vec<(i32, i32)> = rows.into_iter().zip(cols).collect();
        sequence
    }

    // Identify all the connected components using breadth-first-search to this current start location
    // (we have already tagged this as a single island)
    pub fn bfs(matrix: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, sequence: &Vec<(i32, i32)>, start: (i32, i32), size: (i32, i32)) {

        // Essentially, find the connected components related to the current location and add them
        // to the backlog queue for processing until there are no more connected components

        let mut backlog: VecDeque<(i32, i32)> = VecDeque::new();
        backlog.push_back(start); // seed our queue

        // Closure captures matrix and size
        // Safe to visit?
        // Check if positions are valid on the board
        // Check if we're on land '1' or water '0'
        let visitable = |(r, c)| {
            if r >= 0 && r < size.0 && c >= 0 && c < size.1 &&
                matrix[r as usize][c as usize] == '1' {
                    return true
                } else {
                    return false
                }
        };

        let mut peer;

        while !backlog.is_empty() {
            let current = backlog.pop_front().unwrap();

            // check peers (see if they're on the same island as us! may day! may day!)
            for offset in sequence {
                // Add offset into current to get peer locations (connected components)
                peer = (current.0 + offset.0, current.1 + offset.1);

                // If not already visited and safe to visit,
                // Enqueue to backlog and mark as visited
                if visitable(peer) && !visited[peer.0 as usize][peer.1 as usize] {
                    visited[peer.0 as usize][peer.1 as usize] = true;
                    backlog.push_back(peer);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0200() {
        let grid1 = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];

        let grid2 = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];

        assert_eq!(1, Solution::run(&grid1));
        assert_eq!(3, Solution::run(&grid2));
    }
}
