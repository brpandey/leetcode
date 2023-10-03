
/*
90. Subsets II
Medium
9.1K
255
Companies

Given an integer array nums that may contain duplicates, return all possible
subsets
(the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.



Example 1:

Input: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]

Example 2:

Input: nums = [0]
Output: [[],[0]]

*/

pub struct Solution {

}

impl Solution {
    pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();

        Solution::dfs(0, nums, &mut subset, &mut result);
        result.sort_by(|x, y| x.len().cmp(&y.len()));
        result
    }

    pub fn dfs(mut i: usize, nums: &[i32], subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            result.push(subset.clone());
            return
        }

        subset.push(nums[i]);  // include current number
        Self::dfs(i+1, nums, subset, result); // recursive call tree branch left with next number

        while i+1 < nums.len() && nums[i] == nums[i+1] { // if we've seen this value before don't recurse again on it
            i = i+1;
        }

        subset.pop();  // do not include (skip) current number
        Self::dfs(i+1, nums, subset, result) // recursive call tree branch right with next non-duplicate number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0090() {
        let check = vec![vec![], vec![1], vec![2], vec![1, 2], vec![2, 2], vec![1, 2, 2]];
        assert_eq!(check, Solution::subsets(&vec![1,2,2]));
    }
}
