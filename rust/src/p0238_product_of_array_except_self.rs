/*
 * 238. Product of Array Except Self
Medium

Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.

Example 1:

Input: nums = [1,2,3,4]
Output: [24,12,8,6]

Example 2:

Input: nums = [-1,1,0,-3,3]
Output: [0,0,9,0,0]


Constraints:

    2 <= nums.length <= 105
    -30 <= nums[i] <= 30
    The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)

 */


pub struct Solution0 {}

impl Solution0 {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        //     0   1  2  3    indices

        //    [1,  2, 3, 4]   input
        //    [24,12, 8, 6]   output

        //    [1,  1, 2, 6]   mult_from_start of input (A)
        //                    shifted so current element's value is not included only product of everything before current
        //    [24,12, 4, 1]   mult_from_end   of input (B)
        //                    shifted so current element's value is not included only product of everything after current

        //                    multiply each other:  A*B = output
        // ----------------
        //    [24,12, 8, 6]

        let len = nums.len();
        let mut from_start: Vec<i32> = vec![1; len];
        let mut from_end: Vec<i32> = vec![1; len];
        let mut output: Vec<i32> = vec![0;len];

        for n in 1..len {
            from_start[n] = from_start[n-1]*nums[n-1];
        }

        for n in (0..len-1).rev() { // if len 4, 0..3 or 0,1,2 rev or 2,1,0
            from_end[n] = from_end[n+1]*nums[n+1];
        }

        for n in 0..len {
            output[n] = from_start[n] * from_end[n];
        }

        output
    }
}


// use same idea as in Solution 0 but do it in place without auxiliary lists
pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut output: Vec<i32> = vec![1;len];
        let mut from_start_acc = 1;
        let mut from_end_acc = 1;

        for n in 1..len {
            from_start_acc *= nums[n-1];
            output[n] *= from_start_acc;
        }

        for n in (0..len-1).rev() { // if len 4, 0..3 or 0,1,2 rev or 2,1,0
            from_end_acc *= nums[n+1];
            output[n] *= from_end_acc;
        }

        output
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0238() {
        assert_eq!(vec![24,12,8,6], Solution::product_except_self(vec![1,2,3,4]));
        assert_eq!(vec![0,0,9,0,0], Solution::product_except_self(vec![-1,1,0,-3,3]));
    }
}
