/*
 * Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

Example 1:

Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
Example 2:

Input: nums = [0,1]
Output: [[0,1],[1,0]]
Example 3:

Input: nums = [1]
Output: [[1]]

*/

pub struct Solution {}

impl Solution {
    pub fn dfs(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>){
        if (start+1) as usize == nums.len(){
            res.push(nums.clone());
            return;
        }

        for i in start..nums.len(){
            Solution::swap(nums, i, start);
            Solution::dfs(nums, start+1, res);
            Solution::swap(nums, i, start); // undo previous swap
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Solution::dfs(&mut nums.clone(), 0, &mut res);
        res
    }

    pub fn swap(num: &mut Vec<i32>, i: usize, j: usize) {
        if i == j { return }

        if i < num.len() && j < num.len() {
            let temp = num[i];
            num[i] = num[j];
            num[j] = temp;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0046() {
        assert_eq!(vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 2, 1], vec![3, 1, 2]], Solution::permute(vec![1,2,3]));
        assert_eq!(vec![vec![0,1],vec![1,0]], Solution::permute(vec![0,1]));
        assert_eq!(vec![vec![1]], Solution::permute(vec![1]));
    }
}
