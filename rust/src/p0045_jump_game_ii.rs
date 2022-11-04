/*
 * 45. Jump Game II
Medium

You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:

    1 <= j <= nums[i] and
    i + j < n

Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

 

Example 1:

Input: nums = [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:

Input: nums = [2,3,0,1,4]
Output: 2

 
 */

/*
 * Strategy
 *
 * Given vec list e.g. [2,3,1,1,4]
 *
 *                                    0  1  2  3  4
 * want to group by reachable values [2, 3, 1, 1, 4]
 *                                    -  ----  ----
 *                                    S    G1    G2
 *
 * S signifies our starting point, we can jump 1 or 2, 
 * furthest advancing us to G1, from G1, we can advance to G2
 *
 * Basically partition array into these groups that illustrate were the furthest jumps occur
 * 
 */

pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut max_jumped;
        let mut num_jumps = 0;

        while r < nums.len() {
            max_jumped = 0;
            for i in l..=r {
                max_jumped = std::cmp::max(max_jumped, nums[i] as usize + i);
            }

            // create new group partition so to speak
            l = r;
            r = max_jumped;
            num_jumps += 1;
        }

        num_jumps
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0045() {
        assert_eq!(2, Solution::jump(vec![2,3,1,1,4]));
        assert_eq!(2, Solution::jump(vec![2,3,0,1,4]));
    }
}

