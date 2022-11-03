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

/*
Strategy: Use ascii value of words as key, use map to group anagrams
 */

use std::collections::HashMap;

pub struct Solution {}

impl<'a> Solution {
    pub fn run(anagrams: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
        let sum: u16 = 0;
        let mut table: HashMap<u16, Vec<&str>> = HashMap::new();

        // Generate ids for each sequence based on their ascii values, use that as hash table key
        for a in anagrams {
            let key = a.as_bytes().iter().fold(sum, |s, &u| s + u as u16);
            table.entry(key).or_insert_with(|| vec![]).push(a);
        }

        let mut result: Vec<Vec<&str>> = table.values().cloned().collect();
        result.sort(); // So we can assert in tests

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0049() {
        let input: Vec<&str> = vec!["eat","tea","tan","ate","nat","bat"];
        let output: Vec<Vec<&str>> = vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]];

        assert_eq!(Solution::run(&input), output);
        assert_eq!(Solution::run(&vec!["a"]), vec![vec!["a"]]);
        assert_eq!(Solution::run(&vec![""]), vec![vec![""]]);
    }
}
