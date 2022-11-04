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

    /*
        Notes

        Overview
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

        A -

        0123
        If value is "abba" and i is 0 and j is 3, we need to check if ?? is already
        verified as a palindrome as in,"a??a" using our dynamic programming matrix
        if it is, the longer string is also a palindrome
        Since we are populating the dp 2d array bottom up we already have the result

        j-i <=2 condition explanation
        if j is 2 and i is 2 such that j-i is 0 we have a single char "x"
        If j is 3 and i is 2 such that j-i is 1 there's no middle char, "xx"
        If j is 2 and i is 0 such that j-i is 2 we have a middle single char y, "xyx"
     */

pub struct Solution {}

impl Solution {

    pub fn longest_palindrome_substring(value: &str) -> String {
        let len = value.len();
        let mut longest: (usize, usize, usize) = (1, 0, 0);
        let mut dp = vec![vec![false; len]; len]; // Dynamic programming matrix
        let list: Vec<char> = value.chars().collect(); // vectors are index addressable O(1)

        // Iterate across the column
        for j in 0..len {
            // Iterate across the row
            for i in (0..=j).rev() {
                // See A
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

/*
        Evaluating string "c", i 0, j 0
        Found palindrome string "c"
        Evaluating string "b", i 1, j 1
        Found palindrome string "b"
        Evaluating string "cb", i 0, j 1
        Evaluating string "b", i 2, j 2
        Found palindrome string "b"
        Evaluating string "bb", i 1, j 2
        Found palindrome string "bb"
        Evaluating string "cbb", i 0, j 2
        Evaluating string "d", i 3, j 3
        Found palindrome string "d"
        Evaluating string "bd", i 2, j 3
        Evaluating string "bbd", i 1, j 3
        Evaluating string "cbbd", i 0, j 3
*/
        assert_eq!("bb", Solution::longest_palindrome_substring("cbbd"));


/*
        Evaluating string "r", i 0, j 0
        Found palindrome string "r"
        Evaluating string "a", i 1, j 1
        Found palindrome string "a"
        Evaluating string "ra", i 0, j 1
        Evaluating string "c", i 2, j 2
        Found palindrome string "c"
        Evaluating string "ac", i 1, j 2
        Evaluating string "rac", i 0, j 2
        Evaluating string "e", i 3, j 3
        Found palindrome string "e"
        Evaluating string "ce", i 2, j 3
        Evaluating string "ace", i 1, j 3
        Evaluating string "race", i 0, j 3
        Evaluating string "c", i 4, j 4
        Found palindrome string "c"
        Evaluating string "ec", i 3, j 4
        Evaluating string "cec", i 2, j 4
        Found palindrome string "cec"
        Evaluating string "acec", i 1, j 4
        Evaluating string "racec", i 0, j 4
        Evaluating string "a", i 5, j 5
        Found palindrome string "a"
        Evaluating string "ca", i 4, j 5
        Evaluating string "eca", i 3, j 5
        Evaluating string "ceca", i 2, j 5
        Evaluating string "aceca", i 1, j 5
        Found palindrome string "aceca"
        Evaluating string "raceca", i 0, j 5
        Evaluating string "r", i 6, j 6
        Found palindrome string "r"
        Evaluating string "ar", i 5, j 6
        Evaluating string "car", i 4, j 6
        Evaluating string "ecar", i 3, j 6
        Evaluating string "cecar", i 2, j 6
        Evaluating string "acecar", i 1, j 6
        Evaluating string "racecar", i 0, j 6
        Found palindrome string "racecar"
        Evaluating string "s", i 7, j 7
        Found palindrome string "s"
        Evaluating string "rs", i 6, j 7
        Evaluating string "ars", i 5, j 7
        Evaluating string "cars", i 4, j 7
        Evaluating string "ecars", i 3, j 7
        Evaluating string "cecars", i 2, j 7
        Evaluating string "acecars", i 1, j 7
        Evaluating string "racecars", i 0, j 7
         */
        
        assert_eq!("racecar", Solution::longest_palindrome_substring("racecars"));
    }
}
