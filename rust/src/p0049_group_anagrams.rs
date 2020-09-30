/*
49. Group Anagrams
Medium

Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.



Example 1:

Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Example 2:

Input: strs = [""]
Output: [[""]]

Example 3:

Input: strs = ["a"]
Output: [["a"]]



Constraints:

1 <= strs.length <= 104
0 <= strs[i].length <= 100
strs[i] consists of lower-case English letters.


 */

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn run(anagrams: &Vec<String>) -> Vec<Vec<String>> {
        let sum: u16 = 0;
        let mut table: HashMap<u16, Vec<String>> = HashMap::new();

        // Generate ids for each sequence based on their ascii values, use that as hash table key
        for a in anagrams {
            let key = a.as_bytes().iter().fold(sum, |s, &u| s + u as u16);
            table.entry(key).or_insert_with(|| vec![]).push(a.to_owned())
        }

        let mut result: Vec<Vec<String>> = table.values().cloned().collect();
        result.sort(); // So we can assert in tests
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0049() {

        let input: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"].into_iter().map(|s| s.to_owned()).collect();
        let output: Vec<Vec<String>> = vec![vec!["bat".to_owned()], vec!["eat", "tea", "ate"].into_iter().map(|s| s.to_owned()).collect(), vec!["tan".to_owned(), "nat".to_owned()]];
        assert_eq!(Solution::run(&input), output);

        let input: Vec<String> = vec!["a".to_owned()];
        assert_eq!(Solution::run(&input), vec![vec!["a".to_owned()]]);

        assert_eq!(Solution::run(&vec!["".to_owned()]), vec![vec!["".to_owned()]]);
    }
}
