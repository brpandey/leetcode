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

pub struct Solution1 {}

impl Solution1 {
    // For example assume nums is [1,2,3]
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> { Self::permute0(&mut nums) }
    pub fn permute0(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        if nums.len() == 1 { return vec![nums.clone()] }

        for _ in 0..nums.len() {
            let ignore = nums.remove(0); // if we startwed with 1,2,3, now nums is 2,3
            let mut perms = Self::permute0(nums); // perms for 2,3 are => 2,3 and 3,2

            for perm in perms.iter_mut() { // add 1 to end of perms e.g. 2,3,1 and 3,2,1
                perm.push(ignore);
            }

            result.extend(perms);
            nums.push(ignore); // nums now 2,3,1 and ignore for next loop is 2, leaving nums to be 3,1
                               // last loop nums will be 3,1,2 and ignore for next loop is 3, leaving numbs to be 1,2
        }

        return result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    pub fn test_0046() {
        let first_check = vec![vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 2, 1], vec![3, 1, 2]]; 
        assert_eq!(first_check.clone(), Solution::permute(vec![1,2,3]));
        assert_eq!(vec![vec![0,1],vec![1,0]], Solution::permute(vec![0,1]));
        assert_eq!(vec![vec![1]], Solution::permute(vec![1]));



        let first = Solution1::permute(vec![1,2,3]);

        let first_set: HashSet<Vec<i32>> = HashSet::from_iter(first_check.into_iter());
        let second_set = HashSet::from_iter(first);

        assert_eq!(first_set, second_set);
        assert_eq!(vec![vec![1,0],vec![0,1]], Solution1::permute(vec![0,1]));
        assert_eq!(vec![vec![1]], Solution1::permute(vec![1]));

    }
}
