/*
 * 39. Combination Sum
Medium

Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.

It is guaranteed that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

 

Example 1:

Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.

Example 2:

Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]

Example 3:

Input: candidates = [2], target = 1
Output: []

 

Constraints:

    1 <= candidates.length <= 30
    1 <= candidates[i] <= 200
    All elements of candidates are distinct.
    1 <= target <= 500


 */

/* Thoughts
 * Consider example 2
 * 
 * Essentially this can be thought of as a decision tree that doesn't contain duplicates
 * e.g. candidates = [2,3,5], target = 8
 *
 *                        Start
 *                          |
 *            2  ----------/ \--------- [] (No Use of 2)
 *          / |                        /\
 *        2,2 2  (No new 2)           3  [] No use of 3
 *        /  / \                      / \  \
 *   2,2,2  2,3  2,5                3,3 3,5 5
 *      /    /                            
 *2,2,2,2 2,3,3
 *
 * Each level of the decision tree contains the decision either include the value X or not, where X is in
 * cand set e.g. [2,3,5]
 */

pub struct Solution {}

impl Solution {
    pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // Potential nums must be sorted, so we can clean return early (backtrack early knowing we can't find a valid path if proceed early)
        nums.sort();

        let mut result = vec![];
        let mut current = vec![];

        Solution::dfs(0, target, &nums, &mut current, &mut result);

        result
    }

    pub fn dfs(i: usize, acc: i32, nums: &Vec<i32>, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // print A - potential comb

        if acc == 0 { // Base case 1 - found a viable set of numbers in current combination
            result.push(cur.clone());
            return
        }

        if i >= nums.len() || acc < 0 {  // Base case 2
            return // we've exhausted our candidate set, since sorted, next values will only be larger
        }

        cur.push(nums[i]); // include current value, don't advance i, reuse current value (as same number can be used unlimited times)
        Self::dfs(i, acc - nums[i], nums, cur, result);  // recursive call tree branch left with current number (decrementing target)

        cur.pop(); // don't include current value, look at next value
        Self::dfs(i+1, acc, nums, cur, result);  // recursive call tree branch right with next number
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0039() {
        let result: Vec<Vec<i32>> = vec![vec![2,2,3],vec![7]];
        assert_eq!(result, Solution::combination_sum(vec![2,3,6,7], 7));

        let result: Vec<Vec<i32>> = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(result, Solution::combination_sum(vec![2,3,5], 8));

        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(result, Solution::combination_sum(vec![2], 1));
    }
}

/*

Output from second test case, target = 8

potential combination: []
potential combination: [2]
potential combination: [2, 2]
potential combination: [2, 2, 2]
potential combination: [2, 2, 2, 2]
winning combination: [2, 2, 2, 2]
potential combination won't work!: [2, 2, 2] 3
potential combination: [2, 2, 3]
potential combination won't work!: [2, 2, 3] 3
potential combination won't work!: [2, 2] 5
potential combination: [2, 3]
potential combination: [2, 3, 3]
winning combination: [2, 3, 3]
potential combination won't work!: [2, 3] 5
potential combination: [2, 5]
potential combination won't work!: [2, 5] 5
potential combination: [3]
potential combination: [3, 3]
potential combination won't work!: [3, 3] 3
potential combination: [3, 5]
winning combination: [3, 5]
potential combination: [5]
potential combination won't work!: [5] 5


 *                        Start
 *                          |
 *            2  ----------/ \--------- [] (No Use of 2)
 *          / |                        /\
 *        2,2 2  (No new 2)           3  [] No use of 3
 *        /  / \                      / \  \
 *   2,2,2  2,3  2,5                3,3 3,5 5
 *      /    /
 *2,2,2,2 2,3,3


*/
