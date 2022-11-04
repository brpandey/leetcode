/*
217. Contains Duplicate
Easy

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:

Input: nums = [1,2,3,1]
Output: true

Example 2:

Input: nums = [1,2,3,4]
Output: false

Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true
 */

use std::collections::HashSet;

pub struct Solution { }

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {

        let mut set = HashSet::new();

        for x in nums.iter() {
            if set.contains(x) {
                return true
            }

            set.insert(x);
        }

        return false
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0217() {
        assert_eq!(true, Solution::contains_duplicate(vec![1,2,3,1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1,2,3,4]));
        assert_eq!(true, Solution::contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]));
    }
}
