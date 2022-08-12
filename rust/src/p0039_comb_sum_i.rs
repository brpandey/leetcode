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
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // Potential candidates must be sorted, so we can clean return early (backtrack early knowing we can't find a valid path if proceed early)
        candidates.sort();

        let mut result = vec![];
        let mut sequence = vec![];

        Solution::recurse(0, target, &candidates, &mut sequence, &mut result);

        result
    }

    pub fn recurse(index: usize, target: i32, cand: &Vec<i32>, sequence: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
//        println!("potential combination: {:?}", &sequence);

        if target == 0 {
//            println!("winning combination: {:?}", &sequence);
            result.push(sequence.clone());
            return
        }

        for i in index..cand.len() {
            if target < cand[i] {
//                println!("potential combination won't work!: {:?} {:?}", &sequence, &cand[i]);
                return // no valid path possible return early -- backtrack
            }

            let value = cand[i];
            sequence.push(value);
            Solution::recurse(i, target - value, cand, sequence, result);
            sequence.pop();
        }
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
