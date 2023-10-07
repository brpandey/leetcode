/*

72. Edit Distance

Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

You have the following three operations permitted on a word:

Insert a character
Delete a character
Replace a character


Example 1:

Input: word1 = "horse", word2 = "ros"
Output: 3
Explanation: 
horse -> rorse (replace 'h' with 'r')
rorse -> rose (remove 'r')
rose -> ros (remove 'e')
Example 2:

Input: word1 = "intention", word2 = "execution"
Output: 5
Explanation: 
intention -> inention (remove 't')
inention -> enention (replace 'i' with 'e')
enention -> exention (replace 'n' with 'x')
exention -> exection (replace 'n' with 'c')
exection -> execution (insert 'u')


Constraints:

0 <= word1.length, word2.length <= 500
word1 and word2 consist of lowercase English letters.

*/


pub struct Solution {}

impl Solution {
    pub fn edit_distance(word1: &str, word2: &str) -> u32 {
        use std::cmp::min;

        let rows = word1.len();
        let cols = word2.len();

        let first_row = vec![usize::MAX; cols+1];
        let mut dp = vec![first_row; rows+1];

        // Initialize dp matrix bottom row and right most column

        // set bottom row values
        for c in 0..cols+1 {
            dp[rows][c] = cols - c;
        }

        // set rightmost col values
        for r in 0..rows+1 {
            dp[r][cols] = rows - r;
        }

        // start from bottom right of dp matrix and work our way to top left
        for r in (0..rows).rev() {
            for c in (0..cols).rev() {
                // should the characters at each of the words be equal no cost, depend on bottom right diagonal value
                if word1[r..r+1] == word2[c..c+1] {
                    dp[r][c] = dp[r+1][c+1];
                } else {
                    // take previous min operation and 1 for either insert/delete/ or replace operation
                                    // delete in w1   // insert in w1   // replace char in w1
                    dp[r][c] = 1 + min(min(dp[r+1][c], dp[r][c+1]), dp[r+1][c+1]);
                }
            }
        }

        dp[0][0] as u32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0072_edit_distance::tests::test_0072

    #[test]
    pub fn test_0072() {
        let word1 = "horse";
        let word2 = "ros";

        assert_eq!(3, Solution::edit_distance(word1, word2));

        let word1 = "intention";
        let word2 = "execution";

        assert_eq!(5, Solution::edit_distance(word1, word2));
    }
}


