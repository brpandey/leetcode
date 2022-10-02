/*
 * 287. Find the Duplicate Number
Medium

Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.

There is only one repeated number in nums, return this repeated number.

You must solve the problem without modifying the array nums and uses only constant extra space.

 

Example 1:

Input: nums = [1,3,4,2,2]
Output: 2

Example 2:

Input: nums = [3,1,3,4,2]
Output: 3

 

Constraints:

    1 <= n <= 105
    nums.length == n + 1
    1 <= nums[i] <= n
    All the integers in nums appear only once except for precisely one integer which appears two or more times.

 

Follow up:

    How can we prove that at least one duplicate number must exist in nums?
    Can you solve the problem in linear runtime complexity?


 */

pub struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;

        // Use floyd's cycle detection algorithm
        loop {
            // iterate twice for fast
            fast = nums[nums[fast] as usize] as usize;
            // iterate once for fast
            slow = nums[slow] as usize;

            if slow == fast { break }
        }

        let mut slow2 = 0;

        while slow != slow2 {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
        }

        slow as i32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0287() {
        assert_eq!(2, Solution::find_duplicate(vec![1,3,4,2,2]));
        assert_eq!(3, Solution::find_duplicate(vec![3,1,3,4,2]));
    }
}

