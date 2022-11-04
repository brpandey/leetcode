/*
 * 239. Sliding Window Maximum
Hard

You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.

 

Example 1:

Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
Output: [3,3,5,5,6,7]
Explanation: 
Window position                Max
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7

Example 2:

Input: nums = [1], k = 1
Output: [1]

 */

/*
 * Strategy:
 *
 * The queue contains indices not values
 *
 * Given input nums [1,3,-1,-3,5,3,6,7], 
 *
 * Queue is at various points (queue as indices):
 * [0]
 * [1]
 * [1,2]
 * [1,2,3]
 * [4]
 * [4,5]
 * [6]
 * [7]
 *
 * (queue as values)
 *
 * [1]
 * [3]
 * [3,-1]
 * [3,-1,-3]
 * [5]
 * [5,3]
 * [6]
 * [7]
 *
 *
 * grab max value from front of queue o(1) -> [a,b,c] <-- push new value onto back of queue o(1) 
 */

use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // queue of indices, left has the index of those values that are of greater value
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut output: Vec<i32> = Vec::new();
        let mut l = 0;

        for r in 0..nums.len() {
            while !queue.is_empty() && nums[*queue.back().unwrap()] < nums[r] {
                queue.pop_back();
            }

            queue.push_back(r);

            // if the l marker has a higher index than the max value index in the queue, the max
            // value index is old, since its not part of the current window, so pop front in queue
            if l > queue[0] {
                queue.pop_front();
            }

            // if the window size is atleast k, grab the max element from queue
            // max in queue is always at beginning (don't pop just copy)
            // update sliding_window;
            if r - l + 1 >= k as usize {
                output.push(nums[queue[0]]);
                l += 1;
            }
        }

        output
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0239() {
        assert_eq!(vec![3,3,5,5,6,7], Solution::max_sliding_window(vec![1,3,-1,-3,5,3,6,7], 3));
        assert_eq!(vec![1], Solution::max_sliding_window(vec![1], 1));
    }
}

