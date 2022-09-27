/*
 * 36. Valid Sudoku
Medium

Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

    Each row must contain the digits 1-9 without repetition.
    Each column must contain the digits 1-9 without repetition.
    Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

Note:

    A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    Only the filled cells need to be validated according to the mentioned rules.

 

Example 1:

Input: board = 
[["5","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: true

Example 2:

Input: board = 
[["8","3",".",".","7",".",".",".","."]
,["6",".",".","1","9","5",".",".","."]
,[".","9","8",".",".",".",".","6","."]
,["8",".",".",".","6",".",".",".","3"]
,["4",".",".","8",".","3",".",".","1"]
,["7",".",".",".","2",".",".",".","6"]
,[".","6",".",".",".",".","2","8","."]
,[".",".",".","4","1","9",".",".","5"]
,[".",".",".",".","8",".",".","7","9"]]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.

 */


/*
 * Determine uniqueness across rows, cols and subboxes using HashSets
 *
 * Map r,c to subbox with formula (r/3)*3 + c/3
 */

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<&str>>) -> bool {
        let rows = board.len();
        let cols = board[0].len();

        let mut row_sets = vec![HashSet::new(); rows];
        let mut col_sets = vec![HashSet::new(); cols];

        // Grid is 9x9, Hence 9 sub regions of 3x3 regions
        let mut subbox_sets = vec![HashSet::new(); 9];

        for r in 0..rows {
            for c in 0..cols {
                // references are copy types so we're fine
                let val = board[r][c];

                if val == "." { 
                    continue 
                }

                if row_sets[r].contains(&val) || 
                    col_sets[c].contains(&val) ||
                    subbox_sets[(r/3)*3 + c/3].contains(&val) {
                    return false
                } else {
                    row_sets[r].insert(val);
                    col_sets[c].insert(val);
                    subbox_sets[(r/3)*3 + c/3].insert(val);
                }
            }
        }

        return true
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0036() {
        let board1 = vec![
            vec!["5","3",".",".","7",".",".",".","."],
            vec!["6",".",".","1","9","5",".",".","."],
            vec![".","9","8",".",".",".",".","6","."],
            vec!["8",".",".",".","6",".",".",".","3"],
            vec!["4",".",".","8",".","3",".",".","1"],
            vec!["7",".",".",".","2",".",".",".","6"],
            vec![".","6",".",".",".",".","2","8","."],
            vec![".",".",".","4","1","9",".",".","5"],
            vec![".",".",".",".","8",".",".","7","9"]];

        assert_eq!(true, Solution::is_valid_sudoku(board1));

        let board2 = vec![
            vec!["8","3",".",".","7",".",".",".","."],
            vec!["6",".",".","1","9","5",".",".","."],
            vec![".","9","8",".",".",".",".","6","."],
            vec!["8",".",".",".","6",".",".",".","3"],
            vec!["4",".",".","8",".","3",".",".","1"],
            vec!["7",".",".",".","2",".",".",".","6"],
            vec![".","6",".",".",".",".","2","8","."],
            vec![".",".",".","4","1","9",".",".","5"],
            vec![".",".",".",".","8",".",".","7","9"]];

        assert_eq!(false, Solution::is_valid_sudoku(board2));
    }
}

