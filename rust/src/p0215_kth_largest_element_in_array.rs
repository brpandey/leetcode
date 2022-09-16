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
pi           pivot
[3,2,1,5,6,0,4]

          pi    i  pivot   <-- interesting point, since value at i (0) is less than pivot (4), hence swap 0 with pi's value (5)
[3, 2, 1, 5, 6, 0, 4]

             pi    i    <--- swap i and pi's value since loop has been finished, pi needs to demarcate the two sorted halfs wrt to the pivot
[3, 2, 1, 0, 6, 5, 4]

             pi                    target = len(7) - 2 = 5th index,
                                   (pi's index = 4) since 4 < 5, [p+1..r] or [5,6] is next half of list to recurse
[3, 2, 1, 0, 4, 5, 6]

*/

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        Self::quick_select(0, len - 1, len - k as usize, &mut nums)
    }

    pub fn quick_select(l: usize, r: usize, target: usize, nums: &mut Vec<i32>) -> i32 {
        // pi index increases until it hits target index
        let mut pi = l; // partition index
        let pivot = nums[r];
        let mut temp;

        for i in l..r {
            if nums[i] <= pivot {
                // swap nums[i] with nums[pi]
                temp = nums[i];
                nums[i] = nums[pi];
                nums[pi] = temp;
                pi += 1; // keep advancing until num is > than pivot, to swap later on
            }

            // if nums[i] is greater than pivot, the partition_index stays there until
            // there's another nums[i] value that is <= pivot at which point we swap with the last higher value at part_index
        }

        // swap last partition index with the end value, ensuring partition index properly demarcates sorted halves
        temp = nums[pi];
        nums[pi] = nums[r];
        nums[r] = temp;

        if pi > target { // target is behind pi, so look at left half
            Self::quick_select(l, pi - 1, target, nums)
        } else if pi < target {  // target is ahead of pi, so look at right half
            Self::quick_select(pi + 1, r, target, nums)
        } else { // pi == target
            nums[pi]
        }
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

