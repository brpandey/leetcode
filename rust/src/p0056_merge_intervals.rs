/*
 * 56. Merge Intervals
Medium

Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

 

Example 1:

Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

Example 2:

Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.

 

Constraints:

    1 <= intervals.length <= 104
    intervals[i].length == 2
    0 <= starti <= endi <= 104


 *
 */


/*
 * Sort intervals by start element
 * Then 
 */

use std::mem;

pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        // in case clone is not preferred, mem:replace is more efficient
        // though messy looking

        // let mut last: Vec<i32> = intervals[0].clone();
        let mut last: Vec<i32> = mem::replace(&mut intervals[0], vec![]);
        let mut curr: Vec<i32>;

        for i in 1..intervals.len() {
            // curr = intervals[i].clone();
            curr = mem::replace(&mut intervals[i], vec![]);

            // if overlap, need to merge
            if last[1] >= curr[0] {
                last = vec![
                    last[0], std::cmp::max(last[1], curr[1])
                ];
            } else {
                result.push(last);
                last = curr;
            }
        }

        result.push(last);
        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0056() {
        assert_eq!(vec![vec![1,6],vec![8,10],vec![15,18]], Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]));
        assert_eq!(vec![vec![1,5]], Solution::merge(vec![vec![1,4],vec![4,5]]));     
    }
}

