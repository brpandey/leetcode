/*
 * 88. Merge Sorted Array
Easy

You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.

 

Example 1:

Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
Output: [1,2,2,3,5,6]
Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.

Constraints:

    nums1.length == m + n
    nums2.length == n
    0 <= m, n <= 200
    1 <= m + n <= 200
    -109 <= nums1[i], nums2[j] <= 109

 

Follow up: Can you come up with an algorithm that runs in O(m + n) time?

 */

pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut m: usize, nums2: &mut Vec<i32>, mut n: usize) {
        let mut index = m + n - 1;

        while m > 0 && n > 0 {
            // copy greatest element from two numbers into index
            if nums1[m-1] <= nums2[n-1] {
                nums1[index] = nums2[n-1];
                n -= 1; 
            } else {
                nums1[index] = nums1[m-1];
                m -= 1;
            }

            index -= 1;
        }

        // If nums2 still has data make sure that it is copied over
        while n > 0 {
            nums1[index] = nums2[n-1];
            n -= 1;
            if index > 0 { index -= 1};
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0088() {
        let mut input1 = vec![1,2,3,0,0,0];
        let mut input2 = vec![2,5,6];
        Solution::merge(&mut input1, 3, &mut input2, 3);
        assert_eq!(vec![1,2,2,3,5,6], input1);

        let mut input1 = vec![1];
        let mut input2 = vec![];
        Solution::merge(&mut input1, 1, &mut input2, 0);
        assert_eq!(vec![1], input1);

        let mut input1 = vec![0];
        let mut input2 = vec![1];
        Solution::merge(&mut input1, 0, &mut input2, 1);
        assert_eq!(vec![1], input1);
    }
}

