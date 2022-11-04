use std::collections::HashMap;

/*
   https://leetcode.com/problems/two-sum/
   Given an array of integers, return indices of the two numbers such that they add up to a specific target.
   You may assume that each input would have exactly one solution, and you may not use the same element twice.

   Example:

   Given nums = [2, 7, 11, 15], target = 9,

   Because nums[0] + nums[1] = 2 + 7 = 9,
   return [0, 1].
 */

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: &[i32], target: i32) -> Vec<i32> {
        // HashMap<i32, i32> follows copy semantics,
        // keys are values in numbers list, values are indices in numbers list
        let mut map = HashMap::new(); 
        let mut result: Vec<i32> = Vec::new();
        let mut complement: i32;

        for (i, num) in numbers.iter().enumerate() {
            complement = target - num;

            // found a complement
            if let Some(&c_i) = map.get(&complement) {
                result.push(c_i as i32); // complement index
                result.push(i as i32); // original index
                break;
            }

            map.insert(num, i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(vec![0, 1], Solution::two_sum(&vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 4], Solution::two_sum(&vec![7, 1, 10, 22, 14, 6], 15));
        assert_eq!(vec![1, 4], Solution::two_sum(&vec![7, 1, 10, 22, 14, 6, 8], 15));
    }
}
