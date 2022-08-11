/*
 * 377. Combination Sum IV
Medium

Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.

The test cases are generated so that the answer can fit in a 32-bit integer.

 

Example 1:

Input: nums = [1,2,3], target = 4
Output: 7
Explanation:
The possible combination ways are:
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)
Note that different sequences are counted as different combinations.

Example 2:

Input: nums = [9], target = 3
Output: 0

 

Constraints:

    1 <= nums.length <= 200
    1 <= nums[i] <= 1000
    All the elements of nums are unique.
    1 <= target <= 1000

 

Follow up: What if negative numbers are allowed in the given array? How does it change the problem? What limitation we need to add to the question to allow negative numbers?

 *
 *
 *
 */

/*
The possible combination ways are:
1: 4 combinations
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)

2: 2 combinations
(2, 1, 1)
(2, 2)

3: 1 combination
(3, 1)
Note that different sequences are counted as different combinations.

 */

/*
Strategy: Look for patterns given target is 0,1,2,3,4 and nums is [1,2,3]

dp

0 -> 1 // Base Case
1 -> 1 // 1 == 1, also 1-1 is 0, dp[0] is 1, so 1 combination here

So altogether, 1 has 1 combination or dp[0] or dp[1-1]

2 -> 1, [1] => 1 + 1 = 2, also 2-1 is 1 and dp[1] is 1, so 1 combination here
  -> 2,  => 2 == 2, also 2-2 is 0 and dp[0] is 1, so 1 combination here
  so 2 has 2 different combinations

So altogether 2 has 2 combinations or dp[0] + dp[1] or dp[2-2] + dp[2-1]

So now onto 3, what are the different sequences that sum to 3 that start with 1,2,3?
3 -> 3
  -> 2
  -> 1

Notice a few things:

3 -> 3, -- 3 == 3, also 3-3 is 0 and dp[0] is 1, so 1 combination here
     (Notice that after the leading 2 we just pull set from dp[3-2] or dp[1])
  -> 2, [1] -- 2 + 1 = 3, also 3-2 is 1 and dp[1] is 1, so 1 combination here

     (Notice that after the leading 1 we just pull set from dp[3-1] or dp[2] which is two combinations here
  -> 1, [1, 1] -- 1 + 1 + 1 = 3
  -> 1, [2]   -- 1 + 2 = 3


So altogether 3 has 4 combinations or dp[0] + dp[1] + dp[2] or dp[3-3] + dp[3-2] + dp[3-1]

Lastly, 4's combinations can be thought of having
dp[0] + dp[1] + dp[2] + dp[3] combinations = 1 1 2 4
or dp[4-4] + dp[4-3] + dp[4-2] + dp[4-1]

4 ->
     3, [1] -- 3 + 1 = 4, also 4-3 is 1 and dp[1] is 1, so 1 combination here
     2  [1, 1] -- 2 + 1 + 1 = 4, also 4-2 is 1 and dp[2] is 2 so one comb here and one comb on next line
     2  2 -- 2 + 2 = 4

     1 + 3's set, 4-1 is 3 and dp[3] is 4 combinations 

     or
        (notice everything in square brackets is 3's set from above)

     1, [3]
     1, [2, 1]
     1, [1, 1, 1]
     1, [1, 2]

Alternatively you can think of these as paths in a tree with 4 as root node

        4
     /  |  \
    1   2   3
   ... /\   |
      1  2  1
      |
      1
*/


pub struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; target as usize + 1];

        // base case
        dp[0] = 1;

        for i in 1..=target as usize { // e.g. build up dp with targets 1..=4
            for j in 0..nums.len() { // e.g. second loop through 1,2,3

                if i >= nums[j] as usize {
                    dp[i] = dp[i] + dp[i - nums[j] as usize];
                }
            }
        }

        dp[target as usize]
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0377() {
        assert_eq!(7, Solution::combination_sum4(vec![1,2,3], 4));
        assert_eq!(0, Solution::combination_sum4(vec![9], 3));
    }
}

