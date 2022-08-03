/*
 * 417. Pacific Atlantic Water Flow
Medium

There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.

The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).

The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.

Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.

 

Example 1:

Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]

Example 2:

Input: heights = [[2,1],[1,2]]
Output: [[0,0],[0,1],[1,0],[1,1]]


Constraints:

    m == heights.length
    n == heights[r].length
    1 <= m, n <= 200
    0 <= heights[r][c] <= 105


 */

/*
Strategy

        // Mark the rows and columns closest to the oceans part of that ocean's set
        // Do dfs on each cell in the four directions: N, E, S, W
        // and see if the value is greater than the current value

        // To seed it off start with the four regions that are guarenteed to be
        // in a set that are closest to their regions

        // See if eventually those cells show up in both sets
        // (even though the start set might have been different)

        //   P
        // P [ ][ ]
        //   [ ][ ] A
        //       A

        // 1) For the bottom row values, we add to Atlantic set, and look at values
        // a row above, checking if the value is greater than
        // [ ][ ]
        // [X][X]

        // 2) For the top row values, we add to the Pacific set, and look at values a
        // row below, checking if the value is greater than

        // [X][X]
        // [ ][ ]

        // 3) for the first column values, we add to the Pacific set, and look at values
        // a col to the right, check if the value is greater than
        // [X][ ]
        // [X][ ]

        // 4) for the last column values, add to the Atlantic set, and look at values, a col to left
        // check if the value is greater than
        // [ ][X]
        // [ ][X]

        // Keeping track of a visited set to avoid re-doing work, and search the four directions around
        // Do a set intersection of both sets


 */

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let max_rows = heights.len() as i32;
        let max_cols = heights[0].len() as i32;
        let max = (max_rows, max_cols);

        let (mut atlantic, mut pacific) = (HashSet::new(), HashSet::new());

        let prev = -1;

        // horizontal bars
        for n in 0..max_cols as i32 {
            Solution::dfs(0, n, prev, &mut pacific, &heights, max); // Case 2
            Solution::dfs(max_rows-1, n, prev, &mut atlantic, &heights, max); // Case 1
        }

        // vertical bars
        for m in 0..max_rows as i32 {
            Solution::dfs(m, 0, prev, &mut pacific, &heights, max); // Case 3
            Solution::dfs(m, max_cols-1, prev, &mut atlantic, &heights, max); // Case 4
        }

        let mut v = atlantic.intersection(&pacific).into_iter().map(|cell| vec![cell.0, cell.1]).collect::<Vec<Vec<i32>>>();
        v.sort(); // helps with test asserts
        v
    }

    // DFS explores the heights grid space, marking if reachable from given ocean
    pub fn dfs(row: i32, col: i32, prev: i32, reachable: &mut HashSet<(i32, i32)>, heights: &Vec<Vec<i32>>, max: (i32, i32))  {

        let (max_rows, max_cols) = max;
        let (r, c) = (row as usize, col as usize);

        // Make sure hasn't been processed before
        // Ensure current height >= prev property is true otherwise return (climbing up the hill )
        // Sanity check the row/col values
        if reachable.contains(&(row,col)) || row < 0 || row >= max_rows || col < 0 || col >= max_cols || heights[r][c] < prev {
            return
        }

        // Reaching this point indicates that the current cell is reachable from an ocean
        // Since heights[row][col] >= prev

        // reachable is either atlantic or pacific
        reachable.insert((row, col));

        // search in the four directions around cells N, S, W, E (see if next ocean is reachable)
        Solution::dfs(row-1, col, heights[r][c], reachable, heights, max);
        Solution::dfs(row+1, col, heights[r][c], reachable, heights, max);
        Solution::dfs(row, col-1, heights[r][c], reachable, heights, max);
        Solution::dfs(row, col+1, heights[r][c], reachable, heights, max);
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0417() {
        assert_eq!(vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]],
                   Solution::pacific_atlantic(vec![vec![1,2,2,3,5],vec![3,2,3,4,4],vec![2,4,5,3,1],vec![6,7,1,4,5],vec![5,1,1,2,4]]));

        assert_eq!(vec![vec![0,0],vec![0,1],vec![1,0],vec![1,1]], 
                   Solution::pacific_atlantic(vec![vec![2,1],vec![1,2]]));
    }
}

