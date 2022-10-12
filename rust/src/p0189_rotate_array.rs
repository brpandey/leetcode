/*
 * 189. Rotate Array
Medium

Given an array, rotate the array to the right by k steps, where k is non-negative.

Example 1:

Input: nums = [1,2,3,4,5,6,7], k = 3
Output: [5,6,7,1,2,3,4]
Explanation:
rotate 1 steps to the right: [7,1,2,3,4,5,6]
rotate 2 steps to the right: [6,7,1,2,3,4,5]
rotate 3 steps to the right: [5,6,7,1,2,3,4]

Example 2:

Input: nums = [-1,-100,3,99], k = 2
Output: [3,99,-1,-100]
Explanation: 
rotate 1 steps to the right: [99,-1,-100,3]
rotate 2 steps to the right: [3,99,-1,-100]

Constraints:

    1 <= nums.length <= 105
    -231 <= nums[i] <= 231 - 1
    0 <= k <= 105

Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
Could you do it in-place with O(1) extra space?
 */

/*
 * Strategy
 * Approach 1
 *
 * e.g. given below list, with k = 3
 * use formula: j = (i + k) % len, where j is new rotated index
 *                = (i + 3) % 7
 *  0 1 2 3 4 5 6
 * [1,2,3,4,5,6,7]
 *
 * So value 7 at index 6, is mapped to: (6+3)%7 => 9%7 or 2
 * So value 1 at index 0, is mapped to: (0+3)%7 => 3%7 or 3
 *
 *  0 1 2 3 4 5 6
 * [5,6,7,1,2,3,4]
 *
 * Approach 2
 *
 * Reverse entire list once
 * Reverse first 0..k elements, then reverse k..end elements
 *
 * [1,2,3,4,5,6,7]
 * to
 *
 *  0 1 2 3 4 5 6
 * [7,6,5,4,3,2,1]
 *
 * k is 3
 * reverse 0..k and k..len-1
 * [5,6,7,1,2,3,4]
 *
 */

pub struct Solution {}

impl Solution {
    // approach 1
    pub fn rotate0(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let mut nums2 = nums.clone();

        let mut j;
        let k = k as usize % len;

        for i in 0..len {
            // create j index by adding k shift offset and mod-ing length
            j = (i + k) % len;
            nums2[j] = nums[i];
        }

        _ = std::mem::replace(nums, nums2);
    }

    // approach 2
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
       
        //reverse pass 1, 2a, 2b
        Self::reverse(nums, 0, len-1);
        Self::reverse(nums, 0, k-1);
        Self::reverse(nums, k, len-1);       
    }

    pub fn reverse(nums: &mut [i32], mut i: usize, mut j: usize) {
        let mut tmp: i32;

        while i < j {
            tmp = nums[i];
            nums[i] = nums[j];
            nums[j] = tmp;
            i += 1;
            j -= 1;
        }

    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0189() {
        let mut input = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut input, 3);
        assert_eq!(vec![5,6,7,1,2,3,4], input);

        let mut input = vec![-1, -100, 3, 99];
        Solution::rotate(&mut input, 2);
        assert_eq!(vec![3,99,-1,-100], input);
    }
}

