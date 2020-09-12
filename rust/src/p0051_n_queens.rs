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

    pub fn safe(board: &Vec<Vec<u8>>, size: u32, row: u32, column: u32) -> bool {
        // We could invert the loop and search only on diagonals and columns to make more efficient
        // E.g. we feed in only the proper rows that we need to check
        for c in 0..size { // column
            for r in 0..row {
                if board[r as usize][c as usize] == 'Q' as u8 {
                    if c == column { return false }; // if found queen has the same column as our current column => no dice
                    if r as i32 - c as i32 == row as i32 - column as i32 { return false}; // if found queen is on the same \ diagonal => no dice
                    if r + c == row + column { return false}; // if found queen is on the smae / diagonal => no dice
                }
            }
        }

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
            let answer: Vec<String> = board.iter().map(|x| {String::from_utf8(x.to_vec()).unwrap()}).collect();
            solutions.push(answer);
        };

        // Attempt to successfully put Queen on every square in current row
        // When valid spot found, attempt to recurse to place remaining queens
        for col in 0..size {
            if Solution::safe(board, size, row, col) {
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

    #[test]
    pub fn test_0051() {
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

        assert_eq!(Solution::run(4), solution);
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
