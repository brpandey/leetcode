/*
55. Jump Game
Medium

You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.



Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.



Constraints:

1 <= nums.length <= 104
0 <= nums[i] <= 105

*/

/*
 Strategy

Essentially go down the list one by one,
adding the value to the index of the current cell

 0 1 2 3 4          0 1 2 3 4
[2,3,1,1,4]        [3,2,1,0,4]

 2+0 = 2            3+0 = 3
 3+1 = 4            2+1 = 3
 1+2 = 3            1+2 = 3
 1+3 = 4            0+3 = 3
 4+4 = 8

 Hence we keep a running total of the max jump updated at each index
 If we are greater than the current index we can proceed further, if not we can't jump
 
 */

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_jump = 0;

        for (i, &n) in nums.iter().enumerate() {
            max_jump = cmp::max(i+n as usize, max_jump);

            if max_jump <= i {
                return false
            }
        }

        true
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0055() {
        assert_eq!(true, Solution::can_jump(vec![2,3,1,1,4]));
        assert_eq!(false, Solution::can_jump(vec![3,2,1,0,4]));
    }
}

