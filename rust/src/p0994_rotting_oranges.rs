/*
994. Rotting Oranges
Medium

You are given an m x n grid where each cell can have one of three values:

0 representing an empty cell,
1 representing a fresh orange, or
2 representing a rotten orange.

Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange.
If this is impossible, return -1.



Example 1:

Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4

*/

/*
  Use mutli-source bfs, seeding the initial bfs queue with all orange coordinates that are initially value 2

  Then follow standard level - order bfs algorithm, updating time value t to t+1 after level order iteration

  Track if total rotten oranges equals total orange count
*/

use std::collections::{VecDeque, HashSet};

pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let nrows = grid.len();
        let ncols = grid[0].len();
        let mut q: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let mut total = 0; // total oranges count

        for r in 0..nrows {
            for c in 0..ncols {
                if grid[r][c] > 0 {
                    total += 1; // count oranges rotten and unrotten

                    if grid[r][c] == 2 {
                        q.push_back((r, c)); // seed coordinates of rotting oranges
                    }
                }
            }
        }

        let mut rotten_count = 0;
        let mut time = -1;

        let (mut new_r, mut new_c): (i32, i32);
        let (mut nr, mut nc): (usize, usize);

        let (mut level_size, mut infected);

        while !q.is_empty() {
            level_size = q.len();
            rotten_count += level_size;

            for _ in 0..level_size {
                let current = q.pop_front().unwrap();

                for offset in [(-1,0),(1,0),(0,-1),(0,1)] {
                    new_r = current.0 as i32 + offset.0;
                    new_c = current.1 as i32 + offset.1;

                    if new_r >= 0 && new_r < nrows as i32 && new_c >= 0 && new_c < ncols as i32 {
                        (nr, nc) = (new_r as usize, new_c as usize);

                        // if not visited already and orange is not infected yet "1"
                        if !visited.contains(&(nr, nc)) && grid[nr][nc] == 1 {
                            infected = (nr, nc); // orange is now infected
                            q.push_back(infected); // enqueue
                            visited.insert((nr, nc)); // small optimization so we don't revisit nodes we've already seen
                        }
                    }
                }
            }

            time += 1; // after every level order increment time by 1, so on first iter time is 0, and we've processed initial rotten oranges
        }

        if rotten_count == total {
            return time
        } else {
            return -1
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0994_rotting_oranges::tests::test_0994

    #[test]
    pub fn test_0994() {
        let matrix = vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]];
        assert_eq!(4, Solution::oranges_rotting(matrix));

        let matrix = vec![vec![2,1,1],vec![0,1,1],vec![1,0,1]];
        assert_eq!(-1, Solution::oranges_rotting(matrix));

        let matrix = vec![vec![0,2]];
        assert_eq!(0, Solution::oranges_rotting(matrix));
    }
}

