/*
 * 494. Target Sum
Medium

You are given an integer array nums and an integer target.

You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.

    For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".

Return the number of different expressions that you can build, which evaluates to target.

 

Example 1:

Input: nums = [1,1,1,1,1], target = 3
Output: 5
Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3

Example 2:

Input: nums = [1], target = 1
Output: 1

 

Constraints:

    1 <= nums.length <= 20
    0 <= nums[i] <= 1000
    0 <= sum(nums[i]) <= 1000
    -1000 <= target <= 1000


 */

/*
 * Essentially map the problem of finding the combination
 * of adding and subtracting as paths onto a decision tree
 *
 * A decision tree can be represented using recursion
 * Store a cache for each index in the nums list and the current sum or
 * (index,sum), with left means subtracting by 1 and righth means add by 1
 *
 *
 * (l)    0 , 0     (r)
 * -1   /      \    +1
 *   1,-1      1,1
 *   / \      /   \ 
 * 2,-2 2,0  2,0  2,-1
 *        |   |
 *        <---<---- notice (2,0) are the same so we can cache after we compute
 *                  the first time
 *
 * Base case is if the target is met -> return 1 to signify a found solution
 * 2nd Base case is if path len has been exhausted
 *
 * Recursive: Add the results of both branches of the tree
 *
 * Since tree could have height of n, running time would be 2^n
 * So cache results to avoid re-computing
 *
 */

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut cache: HashMap<(usize, i32), i32> = HashMap::new();
        Self::dfs(0, 0, &nums, target, &mut cache)
    }

    pub fn dfs(index: usize, sum: i32, nums: &[i32], target: i32, 
               cache: &mut HashMap<(usize, i32), i32>) -> i32 {

        // Has this key already been precomputed? if so, return precomputed result
        if cache.contains_key(&(index, sum)) {
            return *cache.get(&(index, sum)).unwrap()
        }

        // If we've exhausted the path
        if index == nums.len() {
            // if matching, a single solution has been found
            if sum == target { return 1 } else { return 0 }
        }

        // If the path hasn't been exhausted yet,
        // Build up the cache for key (index, total) by exploring left and right paths
        let value = 
            Self::dfs(index + 1, sum + nums[index], nums, target, cache) +
            Self::dfs(index + 1, sum - nums[index], nums, target, cache);
    
        cache.insert((index, sum), value);

        return cache[&(index, sum)]
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0494() {
        assert_eq!(5, Solution::find_target_sum_ways(vec![1,1,1,1,1], 3));
        assert_eq!(1, Solution::find_target_sum_ways(vec![1], 1));
    }
}

