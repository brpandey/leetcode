/*
 * 953. Verifying an Alien Dictionary
Easy

In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.

Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.

 

Example 1:

Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
Output: true
Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.

Example 2:

Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
Output: false
Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.

Example 3:

Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
Output: false
Explanation: The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character which is less than any other character (More info).

 *
 */

/*
Compare words in pairs - word1, word2, check lexicographic order of the char in word1 and word2
 */

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut map = HashMap::new();
        for (i, b) in order.as_bytes().iter().enumerate() {
            map.insert(*b,i);
        }

        let bytes: Vec<Vec<u8>> = words.iter().map(|s| s.as_bytes().to_owned()).collect();

        for i in 1..bytes.len() {
            let w1 = &bytes[i-1];
            let w2 = &bytes[i];
            let size = std::cmp::min(w1.len(), w2.len());

            for n in 0..size {
                let v1 = map.get(&w1[n]).unwrap();
                let v2 = map.get(&w2[n]).unwrap();

                // if w1's byte is further down the order list than w2's byte, mark as out of order
                // else if w1 has a char which is not the same but less, than no more further chars are
                // necessary to compare as w1 < w2 in lexicographic order

                if v1 > v2 {
                    return false
                } else if v1 < v2 {
                    break;
                }
            }

            // if w2 is a prefix of w1, it needed to be first
            if w1.len() > w2.len() {
                return false
            }
        }

        true
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0953() {
        assert_eq!(true, Solution::is_alien_sorted(vec!["hello".to_owned(),"leetcode".to_owned()], "hlabcdefgijkmnopqrstuvwxyz".to_owned()));
        assert_eq!(false, Solution::is_alien_sorted(vec!["word".to_owned(),"world".to_owned(),"row".to_owned()], "worldabcefghijkmnpqstuvxyz".to_owned()));
        assert_eq!(false, Solution::is_alien_sorted(vec!["apple".to_owned(), "app".to_owned()], "abcdefghijklmnopqrstuvwxyz".to_owned()));

    }
}

