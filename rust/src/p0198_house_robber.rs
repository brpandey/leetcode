/*
 * 198. House Robber
Medium

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

 

Example 1:

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.

Example 2:

Input: nums = [2,7,9,3,1]
Output: 12
Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
Total amount you can rob = 2 + 9 + 1 = 12.

Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 400

 */

use std::cmp;

pub struct Solution {}


/*

_  _  = represents houses that aren't adjacent that we can rob


 0,0,[1,2,3,1]
 _    _         = new_a (0+1) = 1
   _    _       = new_a (0+2) = 2
      _   _     = new_a (1+3) = 4
        _   _   = new_a (2+1) = 3

new_a reflects the longest non_adjacent_sequence sum thus far
new_b reflects the total running max



0,0,[2,7,9,3,1]

_    _         = new_a (0+2) = 2
  _    _       = new_a (0+7) = 7
     _   _     = new_a (2+9) = 11
       _   _   = new_a (7+3) = 10
         _   _ = new_a (11+1) = 12

new_a reflects the longest non_adjacent_sequence sum thus far
new_b reflects the total running max

 */





impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let result = nums.iter().fold((0,0), |(a,b), money| {
            let new_a = b + *money; // new non-adjacent max sequence sum
            let new_b = cmp::max(a,b); // new max from: last non-adjacent max sequence sum and last max
//            println!("new_a (b+money) {}, new_b (max(a,b)) {}, money {}, a {}, b {}", &new_a, &new_b, money, a, b);

            (new_a, new_b)
        });

        cmp::max(result.0, result.1)
    }
}

pub struct Solution2 {}

// Most intuitive but extra storage space
impl Solution2 {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        match len {
            1 => return nums[0],
            // can only take one since they are adjacent, whichever is max
            2 => return cmp::max(nums[0], nums[1]),
            _ => {
                let mut dp = vec![0; len];
                dp[0] = nums[0];
                dp[1] = cmp::max(nums[0], nums[1]);
                for i in 2..len {
                    // The two choices are the current nums value plus the dp value -2 ago (left adjacent to current left adjacent)
                    // or the last dp value from left adjacent
                    // whichever is the maximum

                    // e.g.
                    //      i
                    // [1,2,3]     [1,2,3]
                    //  -   -   or    -

                    // mark the maximum rob value seen thus far at index i
                    dp[i] = cmp::max(nums[i] + dp[i - 2], dp[i - 1]);
                }

                dbg!(&dp);

                return dp[len-1]
            }
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0198() {
        assert_eq!(4, Solution::rob(vec![1,2,3,1]));
        assert_eq!(12, Solution::rob(vec![2,7,9,3,1]));

        assert_eq!(4, Solution2::rob(vec![1,2,3,1]));
        assert_eq!(12, Solution2::rob(vec![2,7,9,3,1]));

    }


    /*
    Test Case 1

    _  _  = represents houses that aren't adjacent that we can rob


        0,0,[1,2,3,1]
        _    _         = new_a (0+1) = 1
          _    _       = new_a (0+2) = 2
             _   _     = new_a (1+3) = 4
                _   _   = new_a (2+1) = 3

        new_b reflects the running max


    new_a (b+money) 1, new_b (max(a,b)) 0, money 1, a 0, b 0
    new_a (b+money) 2, new_b (max(a,b)) 1, money 2, a 1, b 0
    new_a (b+money) 4, new_b (max(a,b)) 2, money 3, a 2, b 1
    new_a (b+money) 3, new_b (max(a,b)) 4, money 1, a 4, b 2
    */


    /*
    Test Case 2
    0,0,[2,7,9,3,1]

    _    _         = new_a (0+2) = 2
      _    _       = new_a (0+7) = 7
         _   _     = new_a (2+9) = 11
           _   _   = new_a (7+3) = 10
             _   _ = new_a (11+1) = 12

        new_b reflects the running max


*/

    /* test case 2
    new_a (b+money) 2, new_b (max(a,b)) 0, money 2, a 0, b 0
    new_a (b+money) 7, new_b (max(a,b)) 2, money 7, a 2, b 0
    new_a (b+money) 11, new_b (max(a,b)) 7, money 9, a 7, b 2
    new_a (b+money) 10, new_b (max(a,b)) 11, money 3, a 11, b 7
    new_a (b+money) 12, new_b (max(a,b)) 11, money 1, a 10, b 11
     */

    /*
    Solution2

    &dp = [
    1,
    2,
    4,
    4,
    ]

    &dp = [
    2,
    7,
    11,
    11,
    12,
]

    */
}

