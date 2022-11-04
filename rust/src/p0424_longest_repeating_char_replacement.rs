/*

424. Longest Repeating Character Replacement
Medium

You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.

Return the length of the longest substring containing the same letter you can get after performing the above operations.



Example 1:

Input: s = "ABAB", k = 2
Output: 4
Explanation: Replace the two 'A's with two 'B's or vice versa.

Example 2:

Input: s = "AABABBA", k = 1
Output: 4
Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
The substring "BBBB" has the longest repeating letters, which is 4.



Constraints:

1 <= s.length <= 105
s consists of only uppercase English letters.
0 <= k <= s.length


 */

/*
Strategy

Given a str like AABABBA and k = 1, create a sliding window that encompasses the first character

AABABBA //  then expand sliding window assuming it follows the invariant of k, recording max window len
-

AABABBA -> AABABBA -> AABABBA 
--         ---        ----

When # of chars to replace exceeds k, e.g. k = 1, shift left edge of sliding window over

AABABBA -> AABABBA -> AABABBA
-----       ----        ---

Continue, expanding sliding window as possible

AABABBA
----

Ultimately the formula to continue expanding window size is:
window_len - count[most_frequent_char] <= k
 */

use std::cmp;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut counts = HashMap::new();
        let mut l = 0; // window left starts at the beginning

        for r in 0..s.len() { // sweep though the str indices, creating a sliding window along with l

            // update map for r's value (r is initially at the beginning just like l)
            *counts.entry(&s[r..r+1]).or_insert(0) += 1;

            while (r - l + 1) - Solution::max_count(&counts) > k as usize {
                *counts.get_mut(&s[l..l+1]).unwrap() -= 1;
                l += 1;
            }

            max_len = cmp::max(r - l + 1, max_len)
        }

        max_len as i32
    }

    pub fn max_count(map: &HashMap<&str, usize>) -> usize {
        let entry = map.iter().max_by_key(|e| e.1).unwrap(); // grab the value of the entry tuple that has the max value
        *entry.1
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0424() {
        assert_eq!(4, Solution::character_replacement(String::from("ABAB"), 2));
        assert_eq!(4, Solution::character_replacement(String::from("AABABBA"), 1));
    }
}

