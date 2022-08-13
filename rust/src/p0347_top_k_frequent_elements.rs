/*
 * 347. Top K Frequent Elements
Medium

Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

 

Example 1:

Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]

Example 2:

Input: nums = [1], k = 1
Output: [1]

 

Constraints:

    1 <= nums.length <= 105
    -104 <= nums[i] <= 104
    k is in the range [1, the number of unique elements in the array].
    It is guaranteed that the answer is unique.

 

Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

 */

/*

Thoughts

// Step 1
// reduce over nums to record the counts
// input [1,1,1,2,2,3] goes to {1: 3, 2: 2, 3: 1} because there are 3 1's, 2 2's, 1 3

// Step 2
// {1: 3, 2: 2, 3: 1} take the counts and flip them into indices  
// {3: [1], 2: [2], 1: [3]}

// then put into vec 0, 1, 2, 3, .... len <--indices but also counts (max count is size len)
//                  [0, 3, 2, 1]      elements that correspond to those counts

//                                                           0  1  2  3  4  5  6
// If nums size 6, was all 3's, the flipped vector would be [0, 0, 0, 0, 0, 0, 3]

*/

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len = nums.len();
        let mut count = 0;
        let mut output: Vec<i32> = vec![];

        // Step 1
        let counts = nums.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

        // Step 2
        let mut flipped_counts = counts.into_iter().fold(HashMap::new(), |mut acc, (k,v)| {
            acc.entry(v).or_insert_with(|| vec![]).push(k);
            acc
        });

        // start from the higher count and return early if we met k
        for i in (1..=len).rev() {
            if let Some(v) = flipped_counts.get_mut(&(i as i32)) {
                while !v.is_empty() {
                    output.push(*v.pop().unwrap());
                    count += 1;

                    if count == k {
                        return output
                    }
                }
            }
        }

        output
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0347() {
        assert_eq!(vec![1,2], Solution::top_k_frequent(vec![1,1,1,2,2,3], 2));
        assert_eq!(vec![1], Solution::top_k_frequent(vec![1], 1));

    }
}

