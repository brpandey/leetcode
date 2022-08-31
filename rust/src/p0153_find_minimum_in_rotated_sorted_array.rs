/*
 * 153. Find Minimum in Rotated Sorted Array
Medium

Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

    [4,5,6,7,0,1,2] if it was rotated 4 times.
    [0,1,2,4,5,6,7] if it was rotated 7 times.

Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

Given the sorted rotated array nums of unique elements, return the minimum element of this array.

You must write an algorithm that runs in O(log n) time.

 

Example 1:

Input: nums = [3,4,5,1,2]
Output: 1
Explanation: The original array was [1,2,3,4,5] rotated 3 times.

Example 2:

Input: nums = [4,5,6,7,0,1,2]
Output: 0
Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.

Example 3:

Input: nums = [11,13,15,17]
Output: 11
Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 

 

Constraints:

    n == nums.length
    1 <= n <= 5000
    -5000 <= nums[i] <= 5000
    All the integers of nums are unique.
    nums is sorted and rotated between 1 and n times.


 */

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let target: i32 = nums[r];
        let mut mid;
        let mut monotonic: bool;

        while l + 1 < r {
            mid = (l + r)/2; // where to split list
            monotonic = nums[mid] < target;

            if monotonic {
                // e.g. 0,1,2,*4,5,6,7 or e.g. 7,0,1,*2,4,5,6
                //            --------               --------
                // go left, find smallest in monotonic range, since everything after mid is bigger
                r = mid;
            } else {
                // e.g. 2,4,5,*6,7,0,1
                //            --------
                // go right, range includes increases 6->7, 0->1 / decreases 7->0, find where it decreases
                l = mid;
            }
        }

        std::cmp::min(nums[l], nums[r])
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0153() {
        assert_eq!(1, Solution::find_min(vec![3,4,5,1,2]));
        assert_eq!(0, Solution::find_min(vec![4,5,6,7,0,1,2]));
        assert_eq!(11, Solution::find_min(vec![11,13,15,17]));
    }
}

