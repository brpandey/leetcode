/*
Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.

Example 1:

Input: "babad"
Output: "bab"
Note: "aba" is also a valid answer.

Example 2:

Input: "cbbd"
Output: "bb"


*/

pub struct Solution {}

impl Solution {
    /*
                        0123
        Given a string "abba", We store true/false if the substring indicated by the start index i
        and the end index j are palindromes as populated from substrings of length 1 up to length
        The substring represented by the indices where i,j = 0,0 is "a"
        The substring represented by the indices where i,j = 1,2 is "bb"
        The substring represented by the indices where i,j = 1,3 is "bba"
        The substring represented by the indices where i,j = 0,3 is "abba"

        (Conceptual substrs being evaluated)      {Actual result of if that substr is a palindrome}
            0    1    2    3     j                    0    1    2    3   j
        0   a   ab   abb   abba                   0   T    F    F    ?
        1        b    bb   bba                    1        T    T    F
        2             b    ba                     2             T    F
        3                  a                      3                  T
        i                                         i

        To evaluate the palindrome value at i,j = 0,3 we check if the first and last characters are the same
        Since "a" == "a" we then check if the dp matrix at i+1,j-1 = 0+1,3-1 = 1,2 is true which it is making
        dp[i,j] = true as well
     */

    pub fn longest_palindrome_substring(value: &str) -> String {
        let len = value.len();
        let mut longest: (usize, usize, usize) = (1, 0, 0);
        let mut dp = vec![vec![false; len]; len]; // Dynamic programming matrix
        let list: Vec<char> = value.chars().collect(); // vectors are index addressable O(1)

        // Iterate across the column
        for j in 0..len {
            // Iterate across the row
            for i in 0..=j {
                //              0123
                // If value is "abba" and i is 0 and j is 3, we need to check if ?? is already
                // verified as a palindrome as in,"a??a" using our dynamic programming matrix
                // if it is, the longer string is also a palindrome
                // Since we are populating the dp 2d array bottom up we already have the result

                // j-i <=2 condition explanation
                // if j is 2 and i is 2 such that j-i is 0 we have a single char "x"
                // If j is 3 and i is 2 such that j-i is 1 there's no middle char, "xx"
                // If j is 2 and i is 0 such that j-i is 2 we have a middle single char y, "xyx"

                if list[i] == list[j] && (j-i <= 2 || dp[i+1][j-1]) {
                    dp[i][j] = true;

                    if (j-i+1, i, j) > longest {
                        longest = (j-i+1, i, j);
                    }
                }
            }
        }

        // Once we've found the proper indices construct the substring into a separate String
        let (_, left, right) = longest;
        // This really only works if the chars are ascii
        let substring = &value[left..=right];
        substring.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005() {
        assert_eq!("b", Solution::longest_palindrome_substring("ab"));
        assert_eq!("aa", Solution::longest_palindrome_substring("aab"));
        assert_eq!("aba", Solution::longest_palindrome_substring("babad"));
        assert_eq!("bb", Solution::longest_palindrome_substring("cbbd"));
        assert_eq!("racecar", Solution::longest_palindrome_substring("racecars"));
    }
}
