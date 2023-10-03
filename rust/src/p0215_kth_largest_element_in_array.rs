/*
215. Kth Largest Element in an Array
Medium

Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

You must solve it in O(n) time complexity.



Example 1:

Input: nums = [3,2,1,5,6,4], k = 2
Output: 5

Example 2:

Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
Output: 4



Constraints:

1 <= k <= nums.length <= 105
-104 <= nums[i] <= 104

*/

/*
 Sorting the list takes o(nlogn)
 For o(n) solution, use quick select
 o(n/2 + n/4 + n/8 ...) (infinite series) => o(n)

 
 */

/*

k = 2

start:

i
j            pivot
[3,2,1,5,6,0,4]

          j     i  pivot   <-- interesting point, since value at i (0) is less than pivot (4), hence swap 0 with j's value (5)
[3, 2, 1, 5, 6, 0, 4]

             j     i    <--- swap i and j's value since loop has been finished, j needs to demarcate the two sorted halfs wrt to the pivot
[3, 2, 1, 0, 6, 5, 4]

             j                    target = len(7) - 2 = 5th index,
                                   (j's index = 4) since 4 < 5, [p+1..r] or [5,6] is next half of list to recurse
[3, 2, 1, 0, 4, 5, 6]

*/

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        Self::quick_select(0, nums.len() - 1, nums.len() - k as usize, &mut nums)
    }

    pub fn quick_select(l: usize, r: usize, target_index: usize, nums: &mut Vec<i32>) -> i32 {
        let mut j = l; // partition index, j points to the last value > pivot
        let pivot = nums[r];

        for i in l..r {
            if nums[i] <= pivot {
                // swap nums[i] with nums[j]
                Self::swap(i, j, nums);
                j += 1; // keep advancing while i points to a value <= pivot
            }

            // if nums[i] is greater than pivot, the partition_index j stays there until
            // there's another nums[i] value that is <= pivot at which point we swap with the last j
        }

        // swap last partition index with the end value, ensuring partition index properly demarcates sorted halves
        // left half being less than value at j and right half greater than value at j
        Self::swap(j, r, nums);
        
        if j > target_index { // target_index is behind j, so look at left half
            Self::quick_select(l, j - 1, target_index, nums)
        } else if j < target_index {  // target_index is ahead of j, so look at right half
            Self::quick_select(j + 1, r, target_index, nums)
        } else { // j == target_index
            nums[j]
        }
    }

    pub fn swap(i: usize, j: usize, nums: &mut [i32]) {
        let temp = nums[i];
        nums[i] = nums[j];
        nums[j] = temp;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0215() {
        assert_eq!(5, Solution::find_kth_largest(vec![3,2,1,5,6,4], 2));
        assert_eq!(4, Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4));
    }
}

