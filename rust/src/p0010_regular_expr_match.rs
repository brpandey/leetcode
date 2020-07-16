/*
10. Regular Expression Matching
Hard

Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.

'.' Matches any single character.
'*' Matches zero or more of the preceding element.

The matching should cover the entire input string (not partial).

Note:

    s could be empty and contains only lowercase letters a-z.
    p could be empty and contains only lowercase letters a-z, and characters like . or *.

Example 1:

Input:
s = "aa"
p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".

Example 2:

Input:
s = "aa"
p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:

Input:
s = "ab"
p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".

Example 4:

Input:
s = "aab"
p = "c*a*b"
Output: true
Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore, it matches "aab".

Example 5:

Input:
s = "mississippi"
p = "mis*is*p*."
Output: false
*/

const ASTERISK: char = '*';
const WILDCARD: char = '.';

pub struct Solution {}

impl Solution {
    pub fn regular_expr_match(text: &str, pattern: &str) -> bool {
        let (m, n) = (text.len(), pattern.len());
        let s: Vec<char> = text.chars().clone().collect();
        let p: Vec<char> = pattern.chars().clone().collect();
        // first create vec of size m+1 (text) and then do that n+1 times (pattern)
        // index into pattern first then text
        let mut dp = vec![vec![false; m+1]; n+1];
        let (mut i, mut j): (usize, usize);

        // Base case
        dp[0][0] = true;

        // Handle initialization case where we have asterisks
        // So if we have a pattern like "a*", "a*b*", "a*b*c*",
        // let's shrink the asterisks to zero occurences mode and pull that value (p can match an empty string text)

        /*
                   0 1 2 3
                   a a a b   s

                 0 1 2 3 4
               0 T F F F F
         0 a   1 F
         1 *   2 T

           p
         */

        for tp in 2..=n {
            // The table index to pattern index mapping is - 1
            i = tp-1;
            if ASTERISK == p[i] {
                dp[tp][0] = dp[tp-2][0];
            }
        }

        /*
                    0 1 2  3   i
                    a a a  b   s

                  0 1 2 3  4   ts
               0  T F F F  F
         0 a   1  F T F F  F
         1 *   2  T T T T  F
                        -- --
         j p   tp

        Note compare the outcomes if the string was aaa (T) vs aaab (F)
         */

        let mut char_before_ast_pat: char;

        // We iterate through the table as well as the text string and pattern at the same time
        // Since this can be confusing =>
        // the table indices are ts, tp
        // the string indices are i, j (after we shift them)
        for ts in 1..=m { // table index string
            for tp in 1..=n { // table index pattern
                // table index to string text and pattern index is shifted -1
                i = ts - 1;
                j = tp - 1;

                if WILDCARD == p[j] || s[i] == p[j] { // Same!
                    // Extend result from previously computed computation
                    // where current s and p each were 1 char prior (effectively diagonal up left)
                    dp[tp][ts] = dp[tp-1][ts-1];
                } else if ASTERISK == p[j] {
                    char_before_ast_pat = p[j-1]; // Grab the char before the asterisk pattern e.g. "a" in "a*"

                    if dp[tp-2][ts] { // If the dp matrix is true (e.g. go up 2) when we use the asterisk pattern to zero occurences, use the result over
                        dp[tp][ts] = true;
                    
                    } else if WILDCARD == char_before_ast_pat || s[i] == char_before_ast_pat {
                        // Carry the previously computed result over when the string current value was the preceding string char
                        // (effectively grab the previously computed value from one left over)
                        dp[tp][ts] = dp[tp][ts-1];
                    }
                }
            }
        }
        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0010(){
        assert_eq!(false, Solution::regular_expr_match("aa", "a"));
        assert_eq!(true, Solution::regular_expr_match("aa", "a*"));
        assert_eq!(true, Solution::regular_expr_match("ab", ".*"));
        assert_eq!(true, Solution::regular_expr_match("aab", "c*a*b"));
        assert_eq!(false, Solution::regular_expr_match("mississippi", "mis*is*p*."));
        assert_eq!(true, Solution::regular_expr_match("abcdb", "a.*b"));
        assert_eq!(true, Solution::regular_expr_match("aaa", "a*"));
        assert_eq!(false, Solution::regular_expr_match("aaab", "a*"));
    }
}
