/*
 * 416. Partition Equal Subset Sum
Medium

Given a non-empty array nums containing only positive integers, find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.

 

Example 1:

Input: nums = [1,5,11,5]
Output: true
Explanation: The array can be partitioned as [1, 5, 5] and [11].

Example 2:

Input: nums = [1,2,3,5]
Output: false
Explanation: The array cannot be partitioned into equal sum subsets.

 */

/*
 * Scan through nums list checking if current element equals target and
 * generate sum combinations e.g.
 *
 * [1,5,11,5]
 *  1      ----> Add to hashset      hs = 1
 *    5, 5+1 -----> Add to hashset   hs = 1,5,6
 *       11, 11+1, 11+5, 11+6 -->    hs = 1,5,6,11,12,16,17 (optimization don't add if v > target)
 *          5, 5+1, 5+5, 5+6 ....
 *
 * An array can be partitioned into two subsets such that the sum of elements in both subsets is
 * equal WHEN we find a sum = sum of elements / 2 = target
 *
 * So the first time 1+5+11+5 = 22, 22/2 => 11 is found terminate, since there has to be another
 * combination of elements (non-overlapping) that sums to 11 since the entire list sum is 22
 */

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        // iter through read set, while writing to write_set 
        // (can't read & write to same set during iter)
        let mut read_set: HashSet<i32> = HashSet::new();
        let mut write_set;

        let sum: i32 = nums.iter().sum();
        let target;

        // sum is odd so unable to get whole halves
        if sum % 2 != 0 {
            return false
        } else {
            target = sum/2;
        }

        for i in 0..nums.len() {
            let current = nums[i];
            write_set = HashSet::new();
            for s in &read_set {
                if current == target || current + *s == target {
                    return true
                }
                write_set.insert(*s); // copy over set element
                write_set.insert(current + *s);
            }

            write_set.insert(current);  // on first iteration, set empty so add current
            read_set = write_set;
        }

        false
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0416() {
        assert_eq!(true, Solution::can_partition(vec![1,5,11,5]));
        assert_eq!(false, Solution::can_partition(vec![1,2,3,5]));
        assert_eq!(true, Solution::can_partition(vec![1,5,10,5,1]));
    }
}

