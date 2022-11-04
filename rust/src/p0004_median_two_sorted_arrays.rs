use std::cmp;
// use tramp::{tramp, Rec};  rust while loop actually faster than simulated tail recursion

/*
    https://leetcode.com/problems/median-of-two-sorted-arrays/

    There are two sorted arrays nums1 and nums2 of size m and n respectively.
    Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
    You may assume nums1 and nums2 cannot be both empty.

    Example 1:

    nums1 = [1, 3]
    nums2 = [2]

    The median is 2.0

    Example 2:

    nums1 = [1, 2]
    nums2 = [3, 4]

    The median is (2 + 3)/2 = 2.5

    https://www.drdobbs.com/parallel/finding-the-median-of-two-sorted-arrays/240169222
    https://www.youtube.com/watch?v=LPFhl65R7ww
 */

/*
Notes/Comments

A -

We use the smallest array - nums1 - as the starting point and try good faith to pick a median value
and see if it will stick otherwise we will refine our guess with other attempts
represented by the tail recursive calls)

Since we are pursuing a more optimal solution than O(n) brute force merge of both lists/arrays,
we construct "two virtual halves" of the two arrays hopefully representing a well matched division
while the elements reside in their original respective sorted arrays

After we choose our partition x, we basically compute whatever partition y value
for nums2 so that we have a basically balanced set
So essentially left half (x + y) is more or less equal right (x + y)

We are able to achieve the log(n+m) runtime efficiency by basically halving these partition values
on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that

B -

Case that confirms that our "bounding box" is correctly chosen --
it is cohesive and well stitched as essentially
everything in the set of x is less than the set of y

The values within the bounding box are exactly in the biggest minimal
middle or median set of the two arrays where we can cleanly pick out
the correct median value while avoid further unnecessary iterations

C -

Note: in sorted array left values are smaller and right values are bigger
Note again: We are able to achieve the log(n+m) runtime efficiency by basically halving these low and high values
on each iteration or partition tail recursive call.  The direct O(1) indexing via arrays enables that

0) low .....P...... high
1) low ..P..px-1

Move left, curent bounding box comprises median values that are too big
(contradicting what a median is), move left and try for smaller values
Tail recursive call, which illustrates the binary search our proportional halving of input (low, partition_x - 1)

D -

Move right, bounding box comprises median values that are too small (contradicting what a median is),
move right and try for larger values
Tail recursive call, which illustrates the binary search our proportional halving of input (partition_x + 1, high)

0) low .....P...... high
1)       px+1..P.. high

*/

pub struct Solution{}

impl Solution {

    fn partition(nums1: &[i32], nums2: &[i32], mut low: usize, mut high: usize, m: usize, n: usize) -> f32 {
        let mut median: f32 = 0.0;

        // See A
        while low <= high {
            let p_x = (low + high) / 2;
            let p_y = (m + n + 1)/2 - p_x;

            // Generate bounding box of four values which represent the median box boundary
            // which are just the corresponding values to the partition value indices

            // Nums1 (x)
            // In the edge cases where max_left has no set of values we use negative inf as a placeholder
            // and similiarily for min_right
            let max_left_x = if p_x > 0 {nums1[p_x-1]} else {i32::MIN};
            let min_right_x = if p_x == m {i32::MAX} else {nums1[p_x]};

            // Nums2 (y)
            // In the edge cases where max_left has no set of values we use negative inf as a placeholder
            // and similiarily for min_right
            let max_left_y = if p_y > 0 {nums2[p_y-1]} else {i32::MIN};
            let min_right_y = if p_y == n {i32::MAX} else {nums2[p_y]};

            /* See B -- Bounding box is correctly chosen */
            if max_left_x <= min_right_y && min_right_x >= max_left_y {
                match (m + n) % 2 == 0 {
                    true => {
                        // Virtual left half and right half across both arrays
                        let v_left = cmp::max(max_left_x, max_left_y);
                        let v_right = cmp::min(min_right_x, min_right_y);
                        median = (v_left + v_right) as f32 / 2.0;
                        return median;
                    },
                    false => {
                        median = cmp::max(max_left_x, max_left_y) as f32;
                        return median;
                    }
                }
            } else if max_left_x > min_right_y {
                /* See C
                0) low .....P...... high
                1) low ..P..px-1
                 */

                high = p_x-1;
            } else {
                /* See D

                0) low .....P...... high
                1)       px+1..P.. high
                */

                low = p_x+1;
            }
        }

        median
    }

    pub fn median_two_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f32 {
        let m: usize = nums1.len();
        let n: usize = nums2.len();

        if m > n {
            Solution::partition(&nums2, &nums1, 0, n, n, m)
        } else {
            Solution::partition(&nums1, &nums2, 0, m, m, n)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0004() {
        assert_eq!(2.0, Solution::median_two_sorted_arrays(&[1,3], &[2]));
        assert_eq!(2.5, Solution::median_two_sorted_arrays(&[1,2], &[3, 40]));
        assert_eq!(4.0, Solution::median_two_sorted_arrays(&[1, 2, 3, 4, 7], &[0, 5, 6, 9]));
        assert_eq!(4.5, Solution::median_two_sorted_arrays(&[1, 2, 3, 4, 7], &[0, 5, 6, 9, 100]));
    }
}
