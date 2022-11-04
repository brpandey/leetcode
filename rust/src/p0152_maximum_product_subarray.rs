/*
152. Maximum Product Subarray
Medium

Given an integer array nums, find a contiguous non-empty subarray within the array that has the largest product, and return the product.

The test cases are generated so that the answer will fit in a 32-bit integer.

A subarray is a contiguous subsequence of the array.

Example 1:

Input: nums = [2,3,-2,4]
Output: 6
Explanation: [2,3] has the largest product 6.

Example 2:

Input: nums = [-2,0,-1]
Output: 0
Explanation: The result cannot be 2, because [-2,-1] is not a subarray.

Constraints:

1 <= nums.length <= 2 * 104
-10 <= nums[i] <= 10
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
*/

use std::cmp;
pub struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return 0 }

        let mut max_prod: i32 = nums[0];

        // these values track the oscillations that happen if we have negative numbers
        let mut curr_min: i32 = 1; // 1 is neutral
        let mut curr_max: i32 = 1; // 1 is neutral
        let mut tmp: i32;

        for &n in nums.iter() {
            tmp = curr_min;

            // we need to weave in n
            // since this is max product we need to multiply our running mins and max with n
            // this handles an even number of negative numbers => which make even output
            // this handles an odd number of negative numbers => which makes odd output
            curr_min = cmp::min(cmp::min(n*curr_max, n*curr_min), n);
            curr_max = cmp::max(cmp::max(n*curr_max, n*tmp), n);

            // pick biggest prod seen so far
            max_prod = cmp::max(max_prod, curr_max);
        }

        max_prod
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0152() {
        assert_eq!(6, Solution::max_product(vec![2,3,-2,4]));
        assert_eq!(0, Solution::max_product(vec![-2,0,-1]));
    }
}
