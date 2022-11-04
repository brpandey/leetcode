/*
213. House Robber II
Medium

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

 

Example 1:

Input: nums = [2,3,2]
Output: 3
Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.

Example 2:

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.

Example 3:

Input: nums = [1,2,3]
Output: 3

 

Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 1000


*/

/*

(House Robber I)

a b -->    a  b  m
0,0,[2, 7, 9, 3, 1]
     2  7 11 11 12 // b values

_ X  _         = (0+2) = 2 > 0
  _  X  _       = (0+7) = 7 > 2
     _  X  _     = (2+9) = 11 > 7
        _  X  _   = (7+3) = 10 < 11
           _  X  _ = (11+1) = 12 > 11


*/

use std::cmp;

pub struct Solution {}

impl Solution {

    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 { return nums[0] }

        // Given a list [1,2,3,4], Given that first and last house are adjacent
        // we take       - - -    as our first sequence
        // and             X X X  as our second sequence
        // and then find the max rob value from each of them

        let seq1 = &nums[0..(n-1)];
        let seq2 = &nums[1..n];

        cmp::max(
            Solution::helper(seq1),
            Solution::helper(seq2)
        )
    }

    pub fn helper(nums: &[i32]) -> i32 {
        let result = nums.iter().fold((0,0), |(mut a, mut b), money| {
            let temp = cmp::max(a + *money, b);
            a = b; // advance a to b
            b = temp; // b takes value for current house we are evaluating
            // it is similiar to the dynamic programming dp value for
            // everything seen up to this point (e.g. up to index i)

            (a, b)
        });

        result.1
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
 
    #[test]
    pub fn test_0213() {
        assert_eq!(3, Solution::rob(vec![2,3,2]));
        assert_eq!(4, Solution::rob(vec![1,2,3,1]));
        assert_eq!(3, Solution::rob(vec![1,2,3]));
    }

    /*

       [2,3,2]
        - -     3
          X X   3
                max is 3

       [1,2,3,1]
        - - -   4 or 1+3
          X X X 3 or 2+1
                max is 4

    */
}

