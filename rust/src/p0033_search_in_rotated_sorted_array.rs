/*
 * 33. Search in Rotated Sorted Array
Medium

There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.

 

Example 1:

Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4 

Example 2:

Input: nums = [4,5,6,7,0,1,2], target = 3
Output: -1

Example 3:

Input: nums = [1], target = 0
Output: -1

 

Constraints:

    1 <= nums.length <= 5000
    -104 <= nums[i] <= 104
    All values of nums are unique.
    nums is an ascending array that is possibly rotated.
    -104 <= target <= 104


 *
 */

/*
 * Strategy
 *
 * On each binary search - problem space halving, determine
 * if mid point is in the left half or not (the rotated part which is greater)
 * or the right half. With such info we want to cleanly check if target
 * lies in a monotonic range, 4-7 is a monotonic range, as is 1-5 ...
 *
 * ^ -- denotes midpoint
 *
 * rotated part start example, target = 0
 *  0  1  2  3  4  5  6
 * [4, 5, 6, 7, 0, 1, 2]
 * -----------
 *           ^
 *                 ^
 *              ^          
 * non rotated part start example, target = 0
 *  0  1  2  3  4  5  6
 * [6, 7, 0, 1, 2, 4, 5]
 *           ----------
 *           ^
 *     ^
 *        ^
 */
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() -1);
        let mut mid;
        let mut rotated_part: bool; // the key observation

        while l <= r {
            // mid provides the binary search aspect, left_half as well
            mid = (l + r) / 2;
            // nums[l] will always contain a rotated value, if nums[mid] is > than it
            // than we know it is also in the rotated part, the non rotated
            // part is less than these values in the rotated part
            rotated_part = nums[mid] >= nums[l];

            if nums[mid] == target {
                return mid as i32
            }

            if rotated_part {
                // check if target lies within range of rotated_part
                if target >= nums[l] && target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                // check if target lies within non-rotated part
                if target > nums[mid] && target <= nums[r] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }

        -1
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0033() {
        assert_eq!(4, Solution::search(vec![4,5,6,7,0,1,2], 0));
        assert_eq!(-1, Solution::search(vec![4,5,6,7,0,1,2], 3));
        assert_eq!(-1, Solution::search(vec![1], 0));
    }
}

