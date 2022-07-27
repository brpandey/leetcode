/*
300. Longest Increasing Subsequence
Medium

Given an integer array nums, return the length of the longest strictly increasing subsequence.

A subsequence is a sequence that can be derived from an array by deleting some or no elements without changing the order of the remaining elements. For example, [3,6,2,7] is a subsequence of the array [0,3,1,6,2,2,7].



Example 1:

Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.

Example 2:

Input: nums = [0,1,0,3,2,3]
Output: 4

Example 3:

Input: nums = [7,7,7,7,7,7,7]
Output: 1



Constraints:

1 <= nums.length <= 2500
-104 <= nums[i] <= 104
*/

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return -1
        }

        let len = nums.len();

        // by default each individual item is an increasing subsequence of itself
        let mut dp = vec![1; len] ;

        //  0 1 2  3  indices
        // [2,5,13,7]
        //         -
        //       - -
        //    -  - -
        //  - -  - -
        // first for loop starts at 7, then 13, then 5, then 2
        // second for loop gather's max value for everything after it

        // initially dp is all 1's
        // [1,1,1, 1]

        // start from the last element to the beginning
        for n in (0..len).rev() {
            // consider the remaining elements after the chosen n looking for a better max value

            for m in n+1..len {
                // since dp is default 1, we are looking to extend our subsequence as long as m's value is greater than n's
                if nums[n] < nums[m] {
                    dp[n] = cmp::max(dp[n], 1 + dp[m]);
                    // println!("dp is {:?}", &dp)
                }
            }
        }

        // grab max value, this then indicates the max length of an increasing subsequence
        dp.into_iter().max().unwrap()
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0300() {
        assert_eq!(4, Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]));
        assert_eq!(4, Solution::length_of_lis(vec![0,1,0,3,2,3]));
        assert_eq!(1, Solution::length_of_lis(vec![7,7,7,7,7,7,7]));
    }
}

/* for first test case

dp is [1, 1, 1, 1, 1, 2, 1, 1]
dp is [1, 1, 1, 1, 1, 2, 1, 1]
dp is [1, 1, 1, 1, 3, 2, 1, 1]
dp is [1, 1, 1, 1, 3, 2, 1, 1]
dp is [1, 1, 1, 1, 3, 2, 1, 1]
dp is [1, 1, 1, 3, 3, 2, 1, 1]
dp is [1, 1, 1, 3, 3, 2, 1, 1]
dp is [1, 1, 1, 3, 3, 2, 1, 1]
dp is [1, 1, 4, 3, 3, 2, 1, 1]
dp is [1, 1, 4, 3, 3, 2, 1, 1]
dp is [1, 1, 4, 3, 3, 2, 1, 1]
dp is [1, 1, 4, 3, 3, 2, 1, 1]
dp is [1, 1, 4, 3, 3, 2, 1, 1]
dp is [1, 2, 4, 3, 3, 2, 1, 1]
dp is [1, 2, 4, 3, 3, 2, 1, 1]
dp is [2, 2, 4, 3, 3, 2, 1, 1]
dp is [2, 2, 4, 3, 3, 2, 1, 1]
*/
