/*
 *
 * 289. Game of Life
Medium

According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

    Any live cell with fewer than two live neighbors dies as if caused by under-population.
    Any live cell with two or three live neighbors lives on to the next generation.
    Any live cell with more than three live neighbors dies, as if by over-population.
    Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.

 

Example 1:

Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]

Example 2:

Input: board = [[1,1],[1,0]]
Output: [[1,1],[1,1]]

 

Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 25
    board[i][j] is 0 or 1.

 

Follow up:

    Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.
    In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?

*/



pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len(); // rows
        let n = board[0].len(); // columns
        let mut peers_alive: i32;

        /*
         * mapping is:
         *
         * orig  new  intermediate
         * 0  ->  0  = 0 // same
         * 1  ->  1  = 1 // same
         * 1  ->  0  = 2
         * 0  ->  1  = 3 // reborn
         *
         * 1, 3 are both odd, and have 1 in the final state
         * 0, 2 are both even, and have 0 in the final state
         */

        for i in 0..m { // rows
            for j in 0..n { // columns

                // for each cell check the life/death status of neighbors
                peers_alive = Solution::check_neighbors(i, j, board);

                // If current cell is alive
                // if neighbors is <2 or >3 die, else lives
                if board[i][j] == 1 {
                    if peers_alive < 2 || peers_alive > 3 {
                        board[i][j] = 2;
                    } // don't need to handle else case as 1 will remain 1

                } else { // if current cell is not alive, check if it can be reborn
                    if peers_alive == 3 {
                        board[i][j] = 3;
                    }
                }
            }
        }


        for i in 0..m { // rows
            for j in 0..n { // columns
//                val = board[i][j];

                if let 2 | 3 = board[i][j] {
                    board[i][j] = board[i][j] % 2 // 2 goes to 0, 3 goes to 1, 
                }
            }
        }

    }

    // Check the 8 neigbors around cells in these directions:
    // NW (-1, -1), N (-1, +0), NE (-1, +1), E (+0, +1), 
    // SE (+1, +1), S (+1, 0), SW (+1, -1), W (0,-1)

    pub fn check_neighbors(row: usize, col: usize, board: &mut Vec<Vec<i32>>) -> i32 {
        let neighbors_offsets: Vec<(i32,i32)> = vec![(-1,-1), (-1,0), (-1,1), (0,1), (1,1), (1,0), (1,-1), (0, -1)];

        neighbors_offsets.iter().fold(0, |acc, (l,r)| {
            acc + Solution::is_alive((row as i32 +*l, col as i32 +*r), board)
        })
    }

    // Check if cell is alive either in original state if it hasn't been modified
    // Or what it was before it was modified
    // A value of 1 means it was originally one or after modification it is still 1
    // A value of 2 means old was 1 and final will be 0, so as of now it is still valid
    pub fn is_alive(cell: (i32, i32), board: &mut Vec<Vec<i32>>) -> i32 {
        let (r, c) = (cell.0 as usize, cell.1 as usize);

        if cell.0 >= 0 && cell.1 >= 0 && r < board.len() && c < board[0].len() &&
            (board[r][c] == 1 || board[r][c] == 2) {
                return 1
            }

        return 0
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0289() {
        let mut board = vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]];
        Solution::game_of_life(&mut board);
        assert_eq!(vec![vec![0,0,0],vec![1,0,1],vec![0,1,1],vec![0,1,0]], board);

        let mut board = vec![vec![1,1],vec![1,0]];
        Solution::game_of_life(&mut board);
        assert_eq!(vec![vec![1,1],vec![1,1]], board);
    }
}



