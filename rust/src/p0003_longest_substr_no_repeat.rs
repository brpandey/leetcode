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


/*
Strategy:
Use l and r as pointers to demarcate our sliding window
Advance r until r's character is a duplicate in the set,
at this point increase l's index rightward until r can proceed

Do all this tracking the longest sequence

//r  (2)
//l  r
//abcabcbb // (2) Need to move l to the left by 1
  ^
  Start, both l and r at the beginning

*/

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // Use sliding window approach and a set
        let mut l = 0;
        let mut max = 0;
        let list: Vec<char> = s.chars().collect();
        let mut unique: HashSet<char> = HashSet::new();

        for r in 0..list.len() {
            // shift left pointer to right by 1, ensuring uniqueness is upheld
            while unique.contains(&list[r]) {
                unique.remove(&list[l]);
                l+=1;
            }
            unique.insert(list[r]);
            max = std::cmp::max(max, r-l+1);
        }

        max as i32
    }
}


pub struct Solution1 {}

impl Solution1 {

    // Note: we use HashMap instead of HashSet to grab the substring order
    pub fn length_of_longest_substring(val: String) -> i32 {
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

        longest_size as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0003() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
    }
}
