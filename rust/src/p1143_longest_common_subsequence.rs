/*

1143. Longest Common Subsequence
Medium

Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

    For example, "ace" is a subsequence of "abcde".

A common subsequence of two strings is a subsequence that is common to both strings.


Example 1:

Input: text1 = "abcde", text2 = "ace"
Output: 3
Explanation: The longest common subsequence is "ace" and its length is 3.

Example 2:

Input: text1 = "abc", text2 = "abc"
Output: 3
Explanation: The longest common subsequence is "abc" and its length is 3.

Example 3:

Input: text1 = "abc", text2 = "def"
Output: 0
Explanation: There is no such common subsequence, so the result is 0.


Constraints:

    1 <= text1.length, text2.length <= 1000
    text1 and text2 consist of only lowercase English characters.
*/

/*
   Thoughts represent the text2 on 1 side of a dynamic programming table e.g. columns, represent text1 on the other side, e.g. rows
   The entries of the dp matrix are numbers indicating the length of the common subsequence up to that point
   e.g.

               column represents base case
               |
               V
      a  c  e
   a  X        0  -- when we reach here, add 1 to max of values around it

   b           0

   c     X     0  -- when we reach here, add 1 to max of values around it

   d           0

   e        X  0  -- start at bottom corner, since e == e, mark this X as 1

   0  0  0  0  0  -- row represents base case



      a  c  e
   a  3        0  -- when we reach here, add 1 to max of values around it

   b           0

   c     2     0  -- when we reach here, add 1 to max of values around it

   d           0

   e  Y  X  1  0  -- start at bottom corner, since e == e, mark this X as 1,
                  -- From X's standpoint which is the intersection of strings "c e" and "e", there is also 1 subsequence of length 1 since e is shared
                  -- from Y's standpoint which is the intersection of strings "a c e" and "e", there is also 1 subsequence of length 1 since e is shared
   0  0  0  0  0


   6X4 DP Table filled in

      a  c  e
   a  3  2  1  0  -- the top left corner is the answer = 3

   b  2  2  1  0

   c  2  2  1  0

   d  1  1  1  0

   e  1  1  1  0

   0  0  0  0  0

 */

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

        let rows = text1.len() + 1; // add 1 to make room for base case row
        let cols = text2.len() + 1; // add 1 to make room for base case col

        let rows_vec: Vec<usize> = vec![0; cols];
        let mut dp = vec![rows_vec; rows];

        // start with bottom row first, last column first
        for i in (0..rows-1).rev() {
            for j in (0..cols-1).rev() {
                if &text1[i..i+1] == &text2[j..j+1] {
                    dp[i][j] = 1 + dp[i+1][j+1]; // grab value from SE cell
                } else {
                    dp[i][j] = cmp::max(dp[i+1][j], dp[i][j+1]); // grab value from S cell and E cell
                }
            }
        }

        dp[0][0] as i32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_1143() {
        assert_eq!(3, Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()));
        assert_eq!(3, Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()));
        assert_eq!(0, Solution::longest_common_subsequence("abc".to_string(), "def".to_string()))
    }

    /*

    Test case 1
    dp is [[3, 2, 1, 0], [2, 2, 1, 0], [2, 2, 1, 0], [1, 1, 1, 0], [1, 1, 1, 0], [0, 0, 0, 0]]
      or just as above

          [[3, 2, 1, 0],
           [2, 2, 1, 0],
           [2, 2, 1, 0],
           [1, 1, 1, 0],
           [1, 1, 1, 0],
           [0, 0, 0, 0]]
    */
}

