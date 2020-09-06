/*
53. Maximum Subarray
Easy

Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.

Example 2:

Input: nums = [1]
Output: 1

Example 3:

Input: nums = [0]
Output: 0

Example 4:

Input: nums = [-1]
Output: -1

Example 5:

Input: nums = [-2147483647]
Output: -2147483647
*/

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn run(nums: &[i32]) -> i32 {
        // Using Kadane's algorithm
        // Described here -> https://medium.com/@rsinghal757/kadanes-algorithm-dynamic-programming-how-and-why-does-it-work-3fd8849ed73d
        let acc = nums
            .iter()
            .fold((i32::min_value(), 0), |(mut global_max, mut local_max), &x| {
                // the local_maximum at index i is the maximum of nums[i] or x
                // and the sum of nums[i] or x and local_maximum at index i-1.
                local_max = cmp::max(x, x + local_max);
                if local_max > global_max { global_max = local_max};
                (global_max, local_max)
            });
        // Since we've folded the entire array this now the global maximum
        acc.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0053() {
        /*
        nums is [-2, 1, -3, 4, -1, 2, 1, -5, 4]
        x is -2, local_max is -2, global_max is -2
        x is +1, local_max is +1, global_max is 1
        x is -3, local_max is -2, global_max is 1
        ---
        x is +4, local_max is +4, global_max is 4
        x is -1, local_max is +3, global_max is 4
        x is +2, local_max is +5, global_max is 5
        x is +1, local_max is +6, global_max is 6
        ---
        x is -5, local_max is +1, global_max is 6
        x is +4, local_max is +5, global_max is 6

        Explanation: [4,-1,2,1] has the largest sum = 6.
        */
        assert_eq!(6, Solution::run(&[-2,1,-3,4,-1,2,1,-5,4]));
        assert_eq!(1, Solution::run(&[1]));
        assert_eq!(0, Solution::run(&[0]));
        assert_eq!(-1, Solution::run(&[-1]));
        assert_eq!(-2147483647, Solution::run(&[-2147483647]));
    }

}
