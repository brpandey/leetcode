/*
 * 647. Palindromic Substrings
Medium

7798

169

Add to List

Share
Given a string s, return the number of palindromic substrings in it.

A string is a palindrome when it reads the same backward as forward.

A substring is a contiguous sequence of characters within the string.

 

Example 1:

Input: s = "abc"
Output: 3
Explanation: Three palindromic strings: "a", "b", "c".
Example 2:

Input: s = "aaa"
Output: 6
Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
 

Constraints:

1 <= s.length <= 1000
s consists of lowercase English letters.
 */



        /*
         * Thoughts
         *
         *               012 (indexes)
         * Given string "aaa"
         *
         * Check two groups of substrings
         *
         * Group 1: Substrings of odd length (1,3,5..)
         * 1       2      3       4
         * _       _     ___      _
         * aaa -> aaa -> aaa -> aaa  
         *
         * Note: the third substring aaa, is a result of expanding after
         * finding the middle a as the second substring)
         * 
         * Group 2: Substrings of even length (2,4,6..)
         * __      __ 
         * aaa -> aaa 
         */



pub struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let sb = s.as_bytes(); // treat string as a slice of bytes so we can index
        let length = sb.len();
        let mut count: i32 = 0;

        for n in 0..length as i32 {
            Solution::helper(&sb, length, n, n, &mut count);
            Solution::helper(&sb, length, n, n+1, &mut count);
        }

        count
    }

    pub fn helper(sb: &[u8], length: usize, mut i: i32, mut j: i32, count: &mut i32) {
        // perform boundary check and that values are equal to each other while updating count
        // expand sliding window based on initial offsets to check for larger substrings that are
        // also palindroms
        //          P          P  P  P
        // e.g. [a][a][a]  -> [a][a][a]
        //          P  P         P  P  P  P
        // e.g. [a][a][a][a] -> [a][a][a][a]
        while i >= 0 && j < length as i32 && sb[i as usize] == sb[j as usize] {
            *count += 1;
            i -= 1;
            j += 1;
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0647() {
        assert_eq!(3, Solution::count_substrings("abc".to_string()));
        assert_eq!(6, Solution::count_substrings("aaa".to_string()));
    }
}

