/*
 * 435. Non-overlapping Intervals
Medium

Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.

 

Example 1:

Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
Output: 1
Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.

Example 2:

Input: intervals = [[1,2],[1,2],[1,2]]
Output: 2
Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.

Example 3:

Input: intervals = [[1,2],[2,3]]
Output: 0
Explanation: You don't need to remove any of the intervals since they're already non-overlapping.

 

Constraints:

    1 <= intervals.length <= 105
    intervals[i].length == 2
    -5 * 104 <= starti < endi <= 5 * 104


 */

pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // Sort intervals vector by start field so we can later compare each pair of entry in one pass
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut last_end = intervals[0][1];
        let mut cnt = 0;

        // essentially doing a fold, with last_end as acc
        for n in 1..intervals.len() {
            let current = &intervals[n];
            if current[0] < last_end { // means overlap encountered
                // given we know an overlap has been found, take greedy approach and keep
                // overlapping interval that ends the soonest, while tallying an extra count
                // for "deleting" the overlap that ends later
                // The intuition is that the smaller overlap will create less overlaps for other
                // elements
                last_end = std::cmp::min(current[1], last_end);
                cnt += 1;
            }
        }

        cnt
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0435() {
        assert_eq!(1, Solution::erase_overlap_intervals(vec![vec![1,2], vec![2,3], vec![3,4], vec![1,3]]));
        assert_eq!(2, Solution::erase_overlap_intervals(vec![vec![1,2], vec![1,2], vec![1,2]]));
        assert_eq!(0, Solution::erase_overlap_intervals(vec![vec![1,2], vec![2,3]]));

    }
}

