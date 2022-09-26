/*
 * 560. Subarray Sum Equals K
Medium

Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

A subarray is a contiguous non-empty sequence of elements within an array.

 

Example 1:

Input: nums = [1,1,1], k = 2
Output: 2

Example 2:

Input: nums = [1,2,3], k = 3
Output: 2

 

Constraints:

    1 <= nums.length <= 2 * 104
    -1000 <= nums[i] <= 1000
    -107 <= k <= 107

 *
 *
 */

/*
 * Strategy:
 * input nums=[1,1,1] k=2
 *
 * map:
 * k    v
 * 0    1
 * 1    1
 * 2    1    First result, sum - k or 2 - 2 = 0, the 0 key has a 1 value
 *           this corresponds to [1,1,X] subarray, 
 * 3    1    since sum - k or 3 - 2 = 1, look at key 1's value which is 1,
 *           so second result, because [X,1,1] is also a valid subarray
 *           Here X is the 1 whose value we are adding to the total sum
 *           
 */

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        // sequential sum counts
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1); // base case
        
        let mut count = 0;
        let mut sum = 0;

        for r in 0..nums.len() {
            sum += nums[r]; // update the running sum
            
            // say sum is 2 e.g. [1,1] and k is 2, sum - k is 0 then
            // so increment count by key 0's count
            //
            // if sum was 3 e.g. [1,1,1] and k is 2, sum - k is 1 then
            // so increment total count by key 1's count
            // e.g. by grabbing key 1's contribution, we are really subtracting out
            // the subarrays that sum to 1 thus far from the subarray [1,1,1] that
            // start at index 0, so [1,1,1] - [1,X,X] gives [X,1,1] so to speak
            count += *map.entry(sum-k).or_insert(0);

            // increment count for sum by 1 in map
            *map.entry(sum).or_insert(0) += 1;
        }

        count as i32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0560() {
        assert_eq!(2, Solution::subarray_sum(vec![1,1,1], 2));
        assert_eq!(2, Solution::subarray_sum(vec![1,2,3], 3));
    }
}

