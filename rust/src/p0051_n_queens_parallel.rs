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

/*
  use std::thread;

Note:
  Using threads without a threadpool breaks down at n = 10 or 724 board solutions
' panicked at 'failed to spawn thread: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', /home/brpandey/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libstd/thread/mod.rs:619:5

:5::55thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/p0051_n_queens_parallel.rs:595
 */

use std::sync::{Arc, Mutex};
use std::time::{Instant};

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
    pub fn solve(solutions_lock: Arc<Mutex<Vec<Vec<String>>>>, board: &mut Vec<Vec<u8>>, size: u32) {

        // Attempt to successfully put Queen on every square in current row
        // When valid spot found, attempt to recurse to place remaining queens
        for col in 0..size {
            // One top-level thread for each spot on the row (for each column)
            // Hence if NxN board we have N threads each doing their own recursive due diligence
            // for a queen on the column spot
            let slock = Arc::clone(&solutions_lock);
            let b = &mut board.to_vec();

            rayon::scope(move |_| {
                // For each thread place queens on each column
                b[0][col as usize] = 'Q' as u8;
                Solution::solve_helper(&slock, b, size, 1);
            });
        }
    }


    // Attempt to solve problem by placing queen in topmost row advancing
    // to the right along all the squares until a spot is found, then
    // recurse for the next row to put the next queen
    pub fn solve_helper(solutions_lock: &Arc<Mutex<Vec<Vec<String>>>>, board: &mut Vec<Vec<u8>>, size: u32, row: u32) {

        // If we have placed all queens successfully on all rows 0..row,
        // the if row == size e.g. 4 means we have successfully finished placing 4 queens
        if row == size {
            let answer: Vec<String> = board.iter().map(|x| {String::from_utf8(x.to_vec()).unwrap()}).collect();
            {
                let mut unlocked = solutions_lock.lock().unwrap();
                unlocked.push(answer);
            }

        };

        // Attempt to successfully put Queen on every square in current row
        // When valid spot found, attempt to recurse to place remaining queens
        for col in 0..size {

            if Solution::safe(&board, size as usize, row as usize, col as usize) {
                // Set queen on current square since there are no threats
                board[row as usize][col as usize] = 'Q' as u8;
                // Attempt to place the remaining queens successfully through recursive calls
                Solution::solve_helper(&solutions_lock, board, size, row + 1);
                // Backtrack and undo the placement of the queen to
                // generate other solution combinations
                board[row as usize][col as usize] = '.' as u8;
            }
        }
    }


    pub fn run(size: u32) -> Vec<Vec<String>>{

        let mut solutions: Vec<Vec<String>> = Vec::with_capacity(size as usize);
        let mut board = vec![vec!['.' as u8; size as usize]; size as usize];

        // Create shared lock
        let solutions_lock: Arc<Mutex<Vec<Vec<String>>>> = Arc::new(Mutex::new(solutions));

        let thread_count = size as usize;
        let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(thread_count).build().unwrap();
        let sl = Arc::clone(&solutions_lock);

        // Ensure that the threadpool has access to the context with which we want to run threads within
        thread_pool.install(|| Solution::solve(sl, &mut board, size));

        // remove solutions value out of Arc and Mutex
        let lock = Arc::try_unwrap(solutions_lock).expect("Unable to shed Arc wrapping as Arc still has multiple owners");
        solutions = lock.into_inner().expect("Mutex lock can not be retrieved");

//        println!("Solutions are {:#?}", solutions);

        solutions
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0051_parallel() {
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
        let duration = start.elapsed();
        println!("N Queens Parallel & Recursive: {:?}", duration);
    }
}
