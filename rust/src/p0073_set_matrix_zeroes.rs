/*
 * 73. Set Matrix Zeroes
Medium

Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

You must do it in place.

 

Example 1:

Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
Output: [[1,0,1],[0,0,0],[1,0,1]]

Example 2:

Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]

 

Constraints:

    m == matrix.length
    n == matrix[0].length
    1 <= m, n <= 200
    -231 <= matrix[i][j] <= 231 - 1

 

Follow up:

    A straightforward solution using O(mn) space is probably a bad idea.
    A simple improvement uses O(m + n) space, but still not the best solution.
    Could you devise a constant space solution?


 *
 */

/*
 *   Given a matrix a)
 *              
 *   a)             Use first row (X's) and col (Y's) 
 *                  as control structure
 *                  with the top left corner (R) 
 *                  "root cell" 
 *
 *   0 1 2 0        R X X X    if we encounter a 0 in the matrix
 *   3 4 5 2        Y 4 5 2    update the relevant control cell in X and Y,
 *   1 0 1 5        Y 0 1 5    e.g. to update the 0 in the last row 2nd col
 *                             update the Y preceding it and the X above it
 *                    *
 * Hence,           0 0 2 0                          0 0 0 0 (Final result)
 *                  3 4 5 2                          0 0 5 0
 *                * 0 0 1 5                          0 0 1 0
 *                  
 *
 *          This runs in-place in O(nm) time
 */

pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut root_cell = false;

        // Establish the X's and Y's from above, setting any to 0 as applicable
        for r in 0..rows {
            for c in 0..cols {
                if 0 == matrix[r][c] {
                    if r == 0 && c == 0 {
                        root_cell = true;
                        continue
                    }

                    // check if cell that has zero is already accounted for 
                    // in both row or col control
                    if matrix[r][0] != 0 {
                        matrix[r][0] = 0;
                    }

                    if matrix[0][c] != 0 {
                        matrix[0][c] = 0;
                    }
                }
            }
        }

        // apply 0 change from the column and row control
        for r in 1..rows {
            for c in 1..cols {
                // if row r at col 0 is 0, zero out row r 
                if matrix[r][0] == 0 {
                    matrix[r][c] = 0
                }

                // if col c at row 0 is 0, zero out col c 
                if matrix[0][c] == 0 {
                    matrix[r][c] = 0
                }
            }
        }

        // step must come after we process the column and row control data
        // set 0 to the first column and first row if true
        if root_cell {
            for c in 0..cols {
                matrix[0][c] = 0;
            }

            for r in 0..rows {
                matrix[r][0] = 0;
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0073() {
        let mut input = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        let output = vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]];

        Solution::set_zeroes(&mut input);
        assert_eq!(output, input);

        let mut input = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
        let output = vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]];
        
        Solution::set_zeroes(&mut input);
        assert_eq!(output, input);
    }
}

