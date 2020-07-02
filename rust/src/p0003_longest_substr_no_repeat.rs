use std::collections::HashMap;

/*
https://leetcode.com/problems/longest-substring-without-repeating-characters/

3. Longest Substring Without Repeating Characters

Given a string, find the length of the longest substring without repeating characters.

Example 1:

Input: "abcabcbb"
Output: 3 
Explanation: The answer is "abc", with the length of 3. 

Example 2:

Input: "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:

Input: "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3. 
Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/

pub struct Solution {}

impl Solution {

    // Note: we use HashMap instead of HashSet to grab the substring order
    pub fn longest_substr_no_repeat(val: &str) -> usize {
        let mut longest_acc: HashMap<char, usize> = HashMap::new();
        let mut current_acc: HashMap<char, usize> = HashMap::new();
        let mut longest_size: usize = 0;
        let mut current_size: usize = 0;

        let mut iter = val.chars().peekable();

        while let Some(k) = iter.next() {
            let mut repeat_peek = false;

            // Is the character in the HashMap?
            // If true reset map and seed with character and count
            // If false, add character c to HashMap increment size

            if current_acc.contains_key(&k) {
                drop(current_acc);
                current_acc = HashMap::new();
                current_acc.insert(k, 0);
                current_size = 1;
            } else {
                current_acc.insert(k, current_size);
                current_size += 1;
            }

            if let Some(&peek) = iter.peek() {
                repeat_peek = current_acc.contains_key(&peek);
            }

            // Check sizes when changing substrings or end of list
            if (iter.peek().is_none() || repeat_peek) &&
                (current_size > longest_size) {
                    drop(longest_acc);
                    longest_acc = current_acc.clone();
                    longest_size = current_size;
                }
        }

//        println!("Explanation: The answer is {:?}, with the length of {} ", longest_acc, longest_size);

        longest_size
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_0003() {
        assert_eq!(3, Solution::longest_substr_no_repeat("pwwkew"));
        assert_eq!(3, Solution::longest_substr_no_repeat("abcabcbb"));
        assert_eq!(1, Solution::longest_substr_no_repeat("bbbbb"));
    }
}
