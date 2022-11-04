/*
 * 167. Two Sum II - Input Array Is Sorted
Medium

Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.
Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution. You may not use the same element twice.

Your solution must use only constant extra space.

Example 1:

Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].

Example 2:

Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].

Example 3:

Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].

 */

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let n = numbers.len();
        let (mut l, mut r) = (0, n-1);

        while l < r {
            let value = numbers[l] + numbers[r];

            if value < target {
                // add a greater number, so move l to right by 1
                l += 1;
            } else if value > target {
                // add a smaller number, so move r to left by 1
                r -= 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1]
            }
        }

        vec![]
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0167() {
        assert_eq!(vec![1,2], Solution::two_sum(vec![2,7,11,15], 9));
        assert_eq!(vec![1,3], Solution::two_sum(vec![2,3,4], 6));
        assert_eq!(vec![1,2], Solution::two_sum(vec![-1,0], -1));
    }
}

