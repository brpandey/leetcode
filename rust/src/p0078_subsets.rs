/*
78. Subsets
Medium

Given an integer array nums of unique elements, return all possible subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.



Example 1:

Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]

Example 2:

Input: nums = [0]
Output: [[],[0]]



Constraints:

1 <= nums.length <= 10
-10 <= nums[i] <= 10
All the numbers of nums are unique.


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

    pub fn dfs(i: usize, nums: &[i32], subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            result.push(subset.clone());
            return
        }

        subset.push(nums[i]);  // include current number
        Self::dfs(i+1, nums, subset, result); // recursive call tree branch left with next number

        subset.pop();  // do not include (skip) current number
        Self::dfs(i+1, nums, subset, result) // recursive call tree branch right with next number
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0078() {
        let answer = vec![vec![],vec![1],vec![2],vec![3],vec![1,2],vec![1,3],vec![2,3],vec![1,2,3]];
        assert_eq!(answer, Solution::subsets(&vec![1,2,3]));
    }
}


/*
subset add is []
subset push is [1]
subset add is [1]
subset push is [1, 2]
subset add is [1, 2]
subset push is [1, 2, 3]
subset add is [1, 2, 3]
subset pop is [1, 2]
subset pop is [1]
subset push is [1, 3]
subset add is [1, 3]
subset pop is [1]
subset pop is []
subset push is [2]
subset add is [2]
subset push is [2, 3]
subset add is [2, 3]
subset pop is [2]
subset pop is []
subset push is [3]
subset add is [3]
subset pop is []
*/
