/*
 * 54. Spiral Matrix
Medium

Given an m x n matrix, return all elements of the matrix in spiral order.

 

Example 1:

Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
Output: [1,2,3,6,9,8,7,4,5]

Example 2:

Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
Output: [1,2,3,4,8,12,11,10,9,5,6,7]

 

Constraints:

    m == matrix.length
    n == matrix[i].length
    1 <= m, n <= 10
    -100 <= matrix[i][j] <= 100

 *
 */

/*
 * Strategy:
 *
 * Maintain a bounding box everytime a row or column is traversed, decrease
 * one of the four parameters of the bounding box!
 *
 */


#[derive(Debug, Copy, Clone)]
pub enum Direction {
    RIGHT = 0,
    DOWN,
    LEFT,
    UP,
}

impl Direction {
    fn toggle(&self) -> Direction {
        use Direction::*;
        match *self {
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            UP => RIGHT,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        // the bounding box params
        let mut left = 0;
        let mut right = cols-1;
        let mut top = 0;
        let mut bottom = rows-1;
        let (mut r, mut c);

        let mut direction = Direction::RIGHT;
        let mut output = Vec::with_capacity(rows*cols);

        while left <= right && top <= bottom {
            match direction {
                Direction::RIGHT => {
                    r = top;
                    for c in left..=right {
                        output.push(matrix[r][c]);
                    }
                    top += 1;
                },
                Direction::DOWN => {
                    c = right;
                    for r in top..=bottom {
                        output.push(matrix[r][c]);
                    }
                    right -= 1;
                },
                Direction::LEFT => {
                    r = bottom;
                    for c in (left..=right).rev() {
                        output.push(matrix[r][c]);
                    }
                    bottom -= 1;
                },
                Direction::UP => {
                    c = left;
                    for r in (top..=bottom).rev() {
                        output.push(matrix[r][c]);
                    }
                    left += 1;
                }
            }

            direction = Direction::toggle(&direction);
        }

        output
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0054() {
        assert_eq!(vec![1,2,3,6,9,8,7,4,5], Solution::spiral_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]));
        assert_eq!(vec![1,2,3,4,8,12,11,10,9,5,6,7], Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]));
    }
}

