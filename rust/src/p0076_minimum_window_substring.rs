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
        // Reduce t into tmap with byte counts, use as a reference
        let tmap = t.as_bytes().iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let (target, mut current) = (tmap.len(), 0);
        let mut window: HashMap<u8, usize> = HashMap::new(); // letter frequencies of current window makeup
        let (mut l, mut r): (usize, usize) = (0, 0);
        let mut min_len = usize::MAX; // start big since target is minimum
        let mut min_indexes = (l,r);
        let (mut leftmost, mut rightmost);
        let bytes = s.as_bytes(); // use byte slice

        // Tally counts by walking source string s, using bytes slice version
        for r in 0..bytes.len() {
            // select byte letter from what r points to (as opposed to l)
            rightmost = bytes.get(r).unwrap();  // let rightmost = &bytes[r..r+1];

            // if rightmost exists in tmap letter counts, must track it, else ignore
            if tmap.contains_key(rightmost) {
                *window.entry(*rightmost).or_insert(0) += 1;

                if window.get(rightmost).unwrap() == tmap.get(rightmost).unwrap() {
                    current += 1;
                }
            }

            // Encourage the search for a shorter sequence, disregard the letter in the beginning of the sequence
            // Continue the rightward expansion while ensuring current == target -- doing this repeatedly
            // Current is the number of target letter matches within current sliding window
            while current == target {
                // after finding valid substring window, record its info if it is less than previous info
                if min_len > r-l+1 {
                    min_indexes = (l, r);
                    min_len = r-l+1;
                }

                // select byte letter in the beginning of the window sequence that l points to for discard
                leftmost = bytes.get(l).unwrap(); 

                // if leftmost byte letter is found in target,
                // decrement current if leftmost's count was equal to its corresponding letter in tmap
                if tmap.contains_key(leftmost) {
                    if tmap.get(leftmost).unwrap() == window.get(leftmost).unwrap() {
                        current -= 1; // decrement current since we're going to remove leftmost letter
                    }

                    *window.get_mut(leftmost).unwrap() -= 1; // reflect changes to window
                }

                l += 1; // move l to the right by 1 (shrink window on left side)
            }
        }

        // check just in case if a substring wasn't found
        if min_len != usize::MAX {
            (l, r) = min_indexes;
            std::str::from_utf8(&bytes[l..=r]).unwrap().to_string()
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

