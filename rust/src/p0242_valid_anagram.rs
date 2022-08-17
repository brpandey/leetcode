/*
242. Valid Anagram
Easy

Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

 

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true

Example 2:

Input: s = "rat", t = "car"
Output: false

 

Constraints:

    1 <= s.length, t.length <= 5 * 104
    s and t consist of lowercase English letters.

 

Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

*/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        // Reduce string to source map
        let mut source = s.as_bytes().iter().fold(HashMap::new(), |mut acc, b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        });

        // Update source map, decrementing counts based on bytes seen in t
        for b in t.as_bytes() {
            if !source.contains_key(b) {
                return false
            }

            source.entry(b).and_modify(|e| *e -= 1); 
        }

        let result = source.into_values().sum();
        //let result = source.into_iter().map(|(_,v)| v).sum();

        if 0 == result { true } else { false }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0242() {
        assert_eq!(true, Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert_eq!(false, Solution::is_anagram("racc".to_string(), "car".to_string()));
        assert_eq!(false, Solution::is_anagram("rat".to_string(), "car".to_string()));

    }
}

/*
running 1 test
[src/p0242_valid_anagram.rs:40] &s = "anagram"
[src/p0242_valid_anagram.rs:41] &t = "nagaram"
[src/p0242_valid_anagram.rs:50] &source = {
    109: 1,
    103: 1,
    114: 1,
    97: 3,
    110: 1,
}
[src/p0242_valid_anagram.rs:61] &source = {
    109: 0,
    103: 0,
    114: 0,
    97: 0,
    110: 0,
}
[src/p0242_valid_anagram.rs:40] &s = "racc"
[src/p0242_valid_anagram.rs:41] &t = "car"
[src/p0242_valid_anagram.rs:50] &source = {
    114: 1,
    97: 1,
    99: 2,
}
[src/p0242_valid_anagram.rs:61] &source = {
    114: 0,
    97: 0,
    99: 1,
}
[src/p0242_valid_anagram.rs:40] &s = "rat"
[src/p0242_valid_anagram.rs:41] &t = "car"
[src/p0242_valid_anagram.rs:50] &source = {
    114: 1,
    97: 1,
    116: 1,
}
*/
