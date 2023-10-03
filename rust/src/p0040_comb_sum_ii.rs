/*
 * 40. Combination Sum II
Medium

Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

Each number in candidates may only be used once in the combination.

Note: The solution set must not contain duplicate combinations.

 

Example 1:

Input: candidates = [10,1,2,7,6,1,5], target = 8
Output: 
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]

Example 2:

Input: candidates = [2,5,2,1,2], target = 5
Output: 
[
[1,2,2],
[5]
]

 */

pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];

        nums.sort();
        Self::dfs(0, target, &nums, &mut current,  &mut result);
        result
    }

    pub fn dfs(mut i: usize, acc: i32, nums: &Vec<i32>, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // values within current add up to target, so add to result
        if acc == 0 {
            result.push(cur.clone());
            return
        }

        // values within current are too big as target has been overshot
        // exhausted possible nums
        if acc < 0 || i >= nums.len() {
            return
        }

        cur.push(nums[i]);
        Self::dfs(i+1, acc - nums[i], nums, cur, result);  // recursive call tree branch left with next number

        // don't select duplicate candidate number again if one is present
        while i+1 < nums.len() && nums[i] == nums[i+1] {  
            i = i+1;
        }

        cur.pop();
        Self::dfs(i+1, acc, nums, cur, result); // recursive call tree branch right with next number (that's not a duplicate)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0040() {
        assert_eq!(vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]], Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8));
        assert_eq!(vec![vec![1, 2, 2], vec![5]], Solution::combination_sum2(vec![2,5,2,1,2], 5));
    }
}

