/*
 * 76. Minimum Window Substring
Hard

Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.

A substring is a contiguous sequence of characters within the string.

 

Example 1:

Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.

Example 2:

Input: s = "a", t = "a"
Output: "a"
Explanation: The entire string s is the minimum window.

Example 3:

Input: s = "a", t = "aa"
Output: ""
Explanation: Both 'a's from t must be included in the window.
Since the largest window of s only has one 'a', return empty string.

 
Constraints:

    m == s.length
    n == t.length
    1 <= m, n <= 105
    s and t consist of uppercase and lowercase English letters.

 */

/*

Thoughts....

Input: s = "ADOBECODEBANC", t = "ABC"


Scan entire string finding each of these sequences that match ABC and recording the min size

Employ a sliding window to find each of these matching subsequences.

Source is what we are matching in this case t, sliding_window or window occurs over s

Move sequences 

         |<>| 3
     |<-->| 2 -- not the smallest
|<-->| 1
ADOBECODEBANC

Designate current as a reflection if we are matching all of source's letter counts reflected in the number target

When current != target, increment right index (since target isn't met expand sequence until it is)
When current == target, increment left index by 1 (essentially we're trying to shorten the sequence since target is met)

ADOBECODEBANC


smallest substring seen so far is Ok("ADOBEC")
smallest substring seen so far is Ok("EBANC")
smallest substring seen so far is Ok("BANC")


*/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // Reduce t into source map with byte counts (using uppercase and lowercase English letters, no foreign character sets)
        // This is what we will be matching against sort of a master or control reference
        let source = t.as_bytes().iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let (target, mut current) = (source.len(), 0);

        // Window counts represent what our min window must have
        let mut window: HashMap<u8, usize> = HashMap::new();

        let (mut l, mut r): (usize, usize) = (0, 0);

        let mut min_len = usize::MAX; // start big since target is minimum
        let mut min_indexes = (l,r);
        let mut first;

        // Tally counts by walking the main string S
        // In order to index into String use bytes slice
        let sb = s.as_bytes();

        for r in 0..sb.len() {
            let letter = sb.get(r).unwrap();
//            let letter = &sb[r..r+1];

            // if letter exists in counts, we need to track it, else ignore
            if source.contains_key(letter) {
                *window.entry(*letter).or_insert(0) += 1;

                if source.get(letter).unwrap() == window.get(letter).unwrap() {
                    current += 1;
                }
            }

            // To encourage the search for a shorter sequence, disregard the letter in the beginning of the sequence
            // and continue the rightward expansion while ensuring current == target -- doing this repeatedly

            // current is a reflection if we are matching all of source's letter counts reflected in the number target
            while current == target {
                // if we've found a valid substring, record its info if it is less than previous info
                if min_len > r-l+1 {
                    min_indexes = (l, r);
                    min_len = r-l+1;
                }

                // disregard the letter in the beginning of the sequence
                first = sb.get(l).unwrap(); 

                // update current if first's count was equal to its corresponding letter in source
                if source.contains_key(first) &&
                    // Check equality before we decrement
                    source.get(first).unwrap() == window.get(first).unwrap() {
                        current -= 1; // decrement current since we're going to decrement first's count
                    }

                // reflect changes to window
                if window.contains_key(first) {
                    *window.get_mut(first).unwrap() -= 1;
                }

                l += 1; // move l to the right by 1
            }
        }

        // check just in case if a substring wasn't found
        if min_len != usize::MAX {
            (l, r) = min_indexes;
            std::str::from_utf8(&sb[l..=r]).unwrap().to_string()
        } else {
            String::new()
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0076() {
        assert_eq!("BANC".to_string(), Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()));
        assert_eq!("a".to_string(), Solution::min_window("a".to_string(), "a".to_string()));
        assert_eq!("".to_string(), Solution::min_window("a".to_string(), "aa".to_string()));
    }
}

