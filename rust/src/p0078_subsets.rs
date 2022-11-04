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

        Solution::generate(0, nums, &mut subset, &mut result);
        result
    }

    pub fn generate(index: usize, nums: &[i32], subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // Print - Add
        result.push(subset.clone());

        for i in index..nums.len() {
            subset.push(nums[i]);
            // Print - Push
            Solution::generate(i + 1, nums, subset, result);
            subset.pop();
            // Print - Pop
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0078() {
        let answer = vec![vec![], vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 3], vec![2], vec![2, 3], vec![3]];
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
