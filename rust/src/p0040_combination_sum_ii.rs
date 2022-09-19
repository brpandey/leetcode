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
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];

        candidates.sort();
        Self::dfs(&candidates, &mut result, &mut current, 0, target);

        result
    }

    pub fn dfs(cand: &Vec<i32>, result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, index: usize, target: i32) {
        // values within current add up to target, so add to result
        if target == 0 {
            result.push(current.clone());
            return
        }

        // values within current are too big as target has been overshot
        if target < 0 {
            return
        }

        let mut last = -1;

        for i in index..cand.len() {
            let v = cand[i];
            if v == last { continue } // skip over duplicates at this level

            current.push(v); // try with value v
            Self::dfs(cand, result, current, i+1, target - v);
            current.pop(); // try without value v
            last = v;
        }
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

