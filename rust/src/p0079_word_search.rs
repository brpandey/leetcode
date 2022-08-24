/*
 * 79. Word Search
Medium

Given an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

 

Example 1:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true

Example 2:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true

Example 3:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false

 
Constraints:

    m == board.length
    n = board[i].length
    1 <= m, n <= 6
    1 <= word.length <= 15
    board and word consists of only lowercase and uppercase English letters.

 */

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<&str>>, word: String) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        let index = 0;
        let mut visited = HashSet::new();

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == &word[index..index+1] {
                    if Self::dfs(index, &word, &mut visited, &board, 
                                 r as i32, c as i32, rows, cols) {
                        return true
                    }
                }
            }
        }

        false
    }

    pub fn dfs(index: usize, word: &str, 
               visited: &mut HashSet<(usize, usize, usize)>, 
               board: &Vec<Vec<&str>>, row: i32, col: i32, 
               rows: usize, cols: usize) -> bool {
 
                // rule out boundary conditions 
        let (r, c) = (row as usize, col as usize);

        println!("r is {}, c is {}, index is {}, board len is {}", r, c, index, board.len());

        if row >= rows as i32 || row < 0 || col >= cols as i32 || col < 0 as i32 
            || board[r][c] != word.get(index..index+1).unwrap() 
            || visited.contains(&(r, c, index)) {
           return false 
        }

        if index + 1 == word.len() {
            return true
        }


        // add r,c coordinates to visited set
        visited.insert((r,c,index));

        // Since there is a matching path, check the 
        // next N, S, W, E cells relative to current r,c cells
        Solution::dfs(index + 1, word, visited, board, row - 1, col, rows, cols) ||
        Solution::dfs(index + 1, word, visited, board, row + 1, col, rows, cols) ||
        Solution::dfs(index + 1, word, visited, board, row, col - 1, rows, cols) ||
        Solution::dfs(index + 1, word, visited, board, row, col + 1, rows, cols)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0079() {
        assert_eq!(true, Solution::exist(
                vec![
                vec!["A","B","C","E"],
                vec!["S","F","C","S"],
                vec!["A","D","E","E"]], "ABCCED".to_string()));

        assert_eq!(true, Solution::exist(
                vec![
                vec!["A","B","C","E"],
                vec!["S","F","C","S"],
                vec!["A","D","E","E"]], "SEE".to_string()));


    }
}

