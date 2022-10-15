/*
 * 41. First Missing Positive
Hard

Given an unsorted integer array nums, return the smallest missing positive integer.

You must implement an algorithm that runs in O(n) time and uses constant extra space.

 

Example 1:

Input: nums = [1,2,0]
Output: 3
Explanation: The numbers in the range [1,2] are all in the array.

Example 2:

Input: nums = [3,4,-1,1]
Output: 2
Explanation: 1 is in the array but 2 is missing.

Example 3:

Input: nums = [7,8,9,11,12]
Output: 1
Explanation: The smallest positive integer 1 is missing.

 

Constraints:

    1 <= nums.length <= 105
    -231 <= nums[i] <= 231 - 1
 */

/*
 * Strategy
 *
 * Input list nums = [3,4,-1,1]
 *
 * Scan list #1) if list contains a 1, if it doesn't return early with value 1
 * as 1 is the first missing positive number that is permissible (0 is neither pos/neg)
 *
 * Scan list #2) if value in list is <= 0 or > len(list) set it to 1
 *
 * Scan list #3) if value in list is in [1..n], then set its value at its supposed sorted index
 * to -1 * current value, as a MARKER to signify value at index is found
 * 
 * index 0 1 2 3                                                                            0 1 2 3
 * e.g. [3,4,1,1] take 3, subract 1 to find its index (2) in the sorted perfect pos version of [1,2,3,4], 
 * update the value at index 2 which is 1 to a -1 to indicate that the 3 is there.
 * 
 * returning this final list state [-3,4,-1,-1] 
 * (we know 1 exists from earlier, so 1-1 is 0 index, change 3 to -3)
 *
 * then find first index that has a non-neg value, which is index 1, add a 1 to it => 2 answer is value 2
 *                                                                      0 1 2 3
 * this corresponds properly to the sorted perfect positive version of [1,2,3,4]
 */

pub struct Solution {}

impl Solution {
    pub fn first_missing_positive(nums: &mut [i32]) -> i32 {
        // Scan 1
        if nums.iter().find(|n| **n == 1).is_none() {
            return 1;
        }

        let len = nums.len();

        // Scan 2
        for i in 0..len {
            if nums[i] <= 0 || nums[i] > len as i32 {
                nums[i] = 1;
            }
        }

        // Scan 3
        let mut ideal_index;
        for i in 0..len {
            // take abs value if this value has already been multiplied
            // by -1 (if it was an original neg number it was already reset to 1)
            ideal_index = (nums[i].abs() - 1) as usize;
            if nums[ideal_index] > 0 {
                // signal that value nums[i] is found in ideal version
                nums[ideal_index] *= -1*nums[ideal_index] as i32
            }
        }

        // Scan 4
        for i in 0..len {
            if nums[i] > 0 {
                // return
                return i as i32 + 1;
            }
        }

        // Otherwise if return hasn't happend yet, it must be n
        len as i32
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0041() {
        let mut input = vec![1,2,0];
        assert_eq!(3, Solution::first_missing_positive(&mut input));
        
        let mut input = vec![3,4,-1,1];
        assert_eq!(2, Solution::first_missing_positive(&mut input));
 
        let mut input = vec![7,8,9,11,12];
        assert_eq!(1, Solution::first_missing_positive(&mut input));
 
    }
}

