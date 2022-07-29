/*
62. Unique Paths
Medium

There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.

Constraints:

1 <= m, n <= 100

 */

pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: usize, n: usize) -> i32 {

        let row: Vec<i32> = vec![0 as i32; n];
        let mut dp: Vec<Vec<i32>> = vec![row; m]; // m rows of vecs size n (col)

        dp[0][0] = 1;

        /*
          Given a 3x2 matrix, there's only 1 way to reach to top of the columns, or the start of each row

          [1][1]
          [1][ ]
          [1][ ]

          [1][1]
          [1][1+1=2]
          [1][1+2=3]

          [1][1]
          [1][2]
          [1][3]

          cells not already initialized can only look at cells, directly above (row-1, col) or directly to the left (row, col-1)
          adding these two path values gives the total number of path values to that cell
        */

        // first row, initialize all col values to 1
        for col in 0..n {
            dp[0][col] = 1
        }

        // first col, initialize all row values to 1
        for row in 0..m {
            dp[row][0] = 1
        }

        for r in 1..m {
            for c in 1..n {
                dp[r][c] = dp[r-1][c] + dp[r][c-1]
            }
        }

        dp[m-1][n-1]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0062() {
        assert_eq!(28, Solution::unique_paths(3,7));
        assert_eq!(3, Solution::unique_paths(3,2));
    }
}


/*
First test case 3x7 -- building on sub-problems

[[1,  1,  1,  1,  1,  1,  1],
 [1,  2,  3,  4,  5,  6,  7],
 [1,  3,  6, 10, 15, 21, 28]]

*/
