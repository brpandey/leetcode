/*

128. Longest Consecutive Sequence
Medium

Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.



Example 1:

Input: nums = [100,4,200,1,3,2]
Output: 4
Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

Example 2:

Input: nums = [0,3,7,2,5,8,4,6,0,1]
Output: 9



Constraints:

0 <= nums.length <= 105
-109 <= nums[i] <= 109


 */

use std::cmp;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        /*
          Given nums [100, 4, 200, 1, 3, 2]

          Visually we know these are the consecutive sequences:

          X,1,2,3,4   X,100    X,200

          The start of sequence has no preceding X or number - 1 e.g. 0, 99, or 199

          While iterating the nums, check for a preceding number given n, if not found,
          check to see if there is a sequence starting at n by looking at next consecutive numbers,
          keeping track of max seq size

          For 1, check 1,2 then 1,2,3 then 1,2,3,4 hence the while loop
        */

        let mut max_length: i32 = 0;

        // create a set representation of nums
        let set: HashSet<&i32> = nums.iter().collect();

        for n in nums.iter() {
            let mut window_size = 1; // a single element has a window size of 1

            // checking n-1 is not in the set indicates potential start of a sequence
            if !set.contains(&(n - 1)) { // n is in the set, check for n+1, n+2...

                while set.contains(&(n+window_size)) {
                    window_size += 1;
                }
            }

            max_length = cmp::max(window_size, max_length);
        }

        max_length
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0128() {
        assert_eq!(4, Solution::longest_consecutive(vec![100,4,200,1,3,2]));
        assert_eq!(9, Solution::longest_consecutive(vec![0,3,7,2,5,8,4,6,0,1]));
    }
}

