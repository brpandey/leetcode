// 48. Rotate Image
//     Medium

//     You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).

//     You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

    

//     Example 1:

// Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//     Output: [[7,4,1],[8,5,2],[9,6,3]]

//     Example 2:

// Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//     Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

//     Example 3:

// Input: matrix = [[1]]
//     Output: [[1]]

//     Example 4:

// Input: matrix = [[1,2],[3,4]]
//     Output: [[3,1],[4,2]]

    

//     Constraints:

// matrix.length == n
//     matrix[i].length == n
//     1 <= n <= 20
//             -1000 <= matrix[i][j] <= 1000

pub struct Solution {}

impl Solution {
    pub fn run(matrix: &mut Vec<Vec<i16>>) {
        let size = matrix.len();

        if matrix[0].len() != size {
            panic!("matrix is not square");
        }
        /*
        Basically there's two dynamics here
        1) The function that maps a cell position matric[r][c] to its destination
        is m[r][c] => m[c][size - 1 - r]

        Here's the table of transitions for the  first input example 1 (m[r][c] => m[c][2-r])
        value      origin       destination
        1          m[0][0]      m[0][2]
        2          m[0][1]      m[1][2]
        3          m[0][2]      m[2][2]
        4          m[1][0]      m[0][1]
        5          m[1][1]      m[1][1]
        6          m[1][2]      m[2][1]
        7          m[2][0]      m[0][0]
        8          m[2][1]      m[1][0]
        9          m[2][2]      m[2][0]

        2) The process with which we swap things over, the easiest way w/o having to use a queue or some auxiliary
        data structure is to use a temporary value, where we take one value out creating a hole, and then swap all the
        transitions into that place for that first "cycle" so to speak
         */

        // Col is dependent on Row so we have separate for loops
        for r in 0..size/2 { // Essentially how many squares (nested concentric squares including outer square)
            for c in r..(size-1-r) { // Start 1 col in extra on every depth essentially shaving off two from the sides each depth

                // Basically these are the transitions for the first cyle for input 1)
                /*
                1 ----> 3
                ^       |
                |       V
                7 <---- 9
                 */

                // 1) Create a hole at the top left corner where 1 originally is
                let mut hole = (r, c);
                let temp = matrix[hole.0][hole.1];

                // 2) Since we are shifting image clockwise by 90, move the closest thing into the hole
                // and then replace that with its antecedent and so on, so basically fill 1's place with 7,
                // fill 7's place with 9, fill 9's place with 3, and fill 3's place with temp
                // destination = f(source)

                // Closure for generating shift sequences
                // size is captured in the closure
                let shift = |(r,c): (usize, usize)| -> (usize, usize) {
                    (c as usize, size-1-r as usize)
                };

                // Shift values between 2d Vector locations
                let apply_shift = |m: &mut Vec<Vec<i16>>, src: (usize, usize), dest: (usize, usize)| {
                    m[dest.0][dest.1] = m[src.0][src.1];
                };

                // Create an iterator of shifts, the first three (given 4 sides of a square)
                // Seq           1    2    3    4
                // Cycle 1: 1 -> 3 -> 9 -> 7 -> 1

                // Cycle 2: 2 -> 6 -> 8 -> 4 -> 2
                // Assume 0 is the location of 1 or (r,c)
                let cycle = (1..4).scan((r,c), |state, _x| {
                    *state = shift(*state);
                    Some(*state)
                }).collect::<Vec<_>>(); // We collect it so that we have a finite collection one that we can reverse

                // We are reversing cycle because to fill the hole
                // we shift in a counter-clockwise fashion (since 1's spot is empty, move 7 in, since 7 is empty move 9 in, etc..)
                // This is opposite of the clockwise rotation

                // Reverse beomes: 3    2    1
                //                 7 -> 9 -> 3

                // The source data now shifted becomes a new hole for the next source
                // 1st iteration: 1 is a hole, it now has 7's contents,
                // 2nd: 7 is now a hole, it now has 9's contents,
                // 3rd: 9 is now a hole, it now has 3's contents
                for &source in cycle.iter().rev() {
                    apply_shift(matrix, source, hole);
                    hole = source;
                }

                matrix[hole.0][hole.1] = temp; // 3 is a hole and it now has 1's original contents
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0048() {
        // 1
        let original: &mut Vec<Vec<i16>> = &mut vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let check: &mut Vec<Vec<i16>> = &mut vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
        Solution::run(original);
        assert_eq!(original, check); // Original vector is modified in place

        // 2
        // Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
        //    Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]

        let original: &mut Vec<Vec<i16>> = &mut vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        let check: &mut Vec<Vec<i16>> = &mut vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]];
        Solution::run(original);
        assert_eq!(original, check); // Original vector is modified in place

        // 3
        let original: &mut Vec<Vec<i16>> = &mut vec![vec![1]];
        let check: &mut Vec<Vec<i16>> = &mut vec![vec![1]];
        Solution::run(original);
        assert_eq!(original, check); // Original vector is modified in place

        // 4
        // Input: matrix = [[1,2],[3,4]]
        //     Output: [[3,1],[4,2]]

        let original: &mut Vec<Vec<i16>> = &mut vec![vec![1,2],vec![3,4]];
        let check: &mut Vec<Vec<i16>> = &mut vec![vec![3,1],vec![4,2]];
        Solution::run(original);
        assert_eq!(original, check); // Original vector is modified in place
    }
}
