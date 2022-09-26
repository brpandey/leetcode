// 51.

// N-Queens
// The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
// Given an integer n, return all distinct solutions to the n-queens puzzle.
// Each solution contains a distinct board configuration of the n-queens' placement,
// where 'Q' and '.' both indicate a queen and an empty space respectively.

// Example:

// Input: 4
// Output: [
//     [".Q..",  // Solution 1
//      "...Q",
//      "Q...",
//      "..Q."],

//     ["..Q.",  // Solution 2
//      "Q...",
//      "...Q",
//      ".Q.."]
// ]

// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.



pub struct Solution {}


impl Solution {

    pub fn safe(board: &Vec<Vec<u8>>, size: usize, row: usize, column: usize) -> bool {
        // Note: We don't need to be concerned with checking rows below us (greater than the current row)

        // Range iterators
        let decr_rows = (0..row).rev();
        let dec_rows = decr_rows.clone();
        let decr_cols = (0..column).rev();
        let incr_cols = (column+1)..size;

        // Does queen exist at row and column location?
        let q = |r: usize, c: usize| {
            if board[r][c] == 'Q' as u8 {
                return true
            } else {
                return false
            }
        };

        // Cases
        // 1) if a queen exists on the same column
        // 2a) if a queen exists on top diagonal left \ (decreasing rows and decreasing columns)
        // 2b) if a queen exists on top diagonal right / (decreasing row and increasing columns)

        for r in 0..row { if q(r, column) { return false }; }; // 1
        for (r,c) in decr_rows.zip(decr_cols) { if q(r, c) { return false }; }; // 2a
        for (r,c) in dec_rows.zip(incr_cols) { if q(r, c) { return false }; }; // 2b

        return true;
    }

    // Attempt to solve problem by placing queen in topmost row advancing
    // to the right along all the squares until a spot is found, then
    // recurse for the next row to put the next queen
    pub fn solve(solutions: &mut Vec<Vec<String>>, board: &mut Vec<Vec<u8>>, size: u32, row: u32) {

        // If we have placed all queens successfully on all rows 0..row,
        // the if row == size e.g. 4 means we have successfully finished placing 4 queens
        if row == size {
            // to_vec does x.iter().cloned().collect::Vector<>()
            // Either slice::to_vec or to_owned() method from the ToOwned trait can be used to create Vec<T> from &[T].
            let answer: Vec<String> = board.iter().map(|x| {String::from_utf8(x.to_vec()).unwrap()}).collect();
            solutions.push(answer);
        };

        // Attempt to successfully put Queen on every square in current row
        // When valid spot found, attempt to recurse to place remaining queens
        for col in 0..size {
            if Solution::safe(board, size as usize, row as usize, col as usize) {
                // Set queen on current square since there are no threats
                board[row as usize][col as usize] = 'Q' as u8;

                // Attempt to place the remaining queens successfully through recursive calls
                Solution::solve(solutions, board, size, row + 1);

                // Backtrack and undo the placement of the queen to
                // generate other solution combinations
                board[row as usize][col as usize] = '.' as u8;
            }
        }
    }

    pub fn run(size: u32) -> Vec<Vec<String>>{
        let mut solutions: Vec<Vec<String>> = Vec::with_capacity(size as usize);
        let mut board = vec![vec!['.' as u8; size as usize]; size as usize];
        Solution::solve(&mut solutions, &mut board, size, 0);  // Start with row 0
        solutions
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant};
    
    #[test]
    pub fn test_0051_recursive() {
        let solution = [
            [
                ".Q..",
                "...Q",
                "Q...",
                "..Q.",
            ],
            [
                "..Q.",
                "Q...",
                "...Q",
                ".Q..",
            ],
        ];


        let start = Instant::now();
        assert_eq!(Solution::run(4), solution);
        let _duration = start.elapsed();

//        println!("N Queens Recursive: {:?}", duration);
    }
}

// 10 solution where size is a 5x5 board or 5 queens
// Solutions = [
//     [
//         "Q....",
//         "..Q..",
//         "....Q",
//         ".Q...",
//         "...Q.",
//     ],
//     [
//         "Q....",
//         "...Q.",
//         ".Q...",
//         "....Q",
//         "..Q..",
//     ],
//     [
//         ".Q...",
//         "...Q.",
//         "Q....",
//         "..Q..",
//         "....Q",
//     ],
//     [
//         ".Q...",
//         "....Q",
//         "..Q..",
//         "Q....",
//         "...Q.",
//     ],
//     [
//         "..Q..",
//         "Q....",
//         "...Q.",
//         ".Q...",
//         "....Q",
//     ],
//     [
//         "..Q..",
//         "....Q",
//         ".Q...",
//         "...Q.",
//         "Q....",
//     ],
//     [
//         "...Q.",
//         "Q....",
//         "..Q..",
//         "....Q",
//         ".Q...",
//     ],
//     [
//         "...Q.",
//         ".Q...",
//         "....Q",
//         "..Q..",
//         "Q....",
//     ],
//     [
//         "....Q",
//         ".Q...",
//         "...Q.",
//         "Q....",
//         "..Q..",
//     ],
//     [
//         "....Q",
//         "..Q..",
//         "Q....",
//         "...Q.",
//         ".Q...",
//     ],
// ]
