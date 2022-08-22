/*
 * 57. Insert Interval
Medium

You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

Return intervals after the insertion.

 

Example 1:

Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,5],[6,9]]

Example 2:

Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].

 *
 */

pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let length = intervals.len();

        for i in 0..length {
            // if the new interval's end is less than interval's list's current entry's start 
            // return new interval along with rest of intervals
            if new_interval[1] < intervals[i][0] {
                result.push(new_interval);
                result.extend_from_slice(&intervals[i..length]);
                return result

            // if the new interval's start is after the interval's list's current entry's end
            // add that current entry to the result and hold on to new_interval
            } else if new_interval[0] > intervals[i][1] {
                result.push(intervals[i].clone());
            // else if the new interval overlaps the current interval's entry, merge the two
            // intervals into new_interval and don't add push to result yet 
            } else {
                new_interval = vec![
                    std::cmp::min(new_interval[0], intervals[i][0]), 
                    std::cmp::max(new_interval[1], intervals[i][1])
                ];
            }
        }

        result.push(new_interval);
        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0057() {
        assert_eq!(vec![vec![1,5], vec![6,9]], 
                   Solution::insert(vec![vec![1,3], vec![6,9]], vec![2,5]));
        assert_eq!(vec![vec![1,2],vec![3,10],vec![12,16]],
                   Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8]));
    }
}

/*
if a is first first if block, b for else if, and c for else, here's the result for second assert_eq:

b
c
c
c
a
*/
