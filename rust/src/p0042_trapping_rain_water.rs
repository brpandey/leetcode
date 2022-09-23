/*
 * 42. Trapping Rain Water
Hard

Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

 

Example 1:

Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.

Example 2:

Input: height = [4,2,0,3,2,5]
Output: 9

 */

/*
 * Strategy
 *
 * For each height value, the solution is governed by the equation
 * water += min(left[i], right[i]) - height[i]
 *
 * Find the side of the structure that most nearly encloses
 * this is the min of the left max, right max values
 *
 * This can also be done in O(1) time memory by using two markers
 * l and r and moving l or r only when l_max is > r_max or vice versa
 *
 * Since min(leftMax, rightMax) hinges on the insight that if leftMax is say smaller
 * e.g. 2 and rightMax is 5, it is fine to keep only looking at leftMax as long as its
 * value is under 5 since the min of the two will always be leftMax
 */

pub struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
            
        let mut left = vec![0; height.len()];
        let mut right = vec![0; height.len()];
        let mut max = 0;

        for i in 0..height.len()-1 {
            max = std::cmp::max(max, height[i]);
            left[i+1] = max;
        }

        max = 0;

        for i in (1..height.len()).rev() {
            max = std::cmp::max(max, height[i]);
            right[i-1] = max;
        }

        let mut x;
        max = 0;

        for i in 0..height.len() {
            x = std::cmp::min(left[i], right[i]);
            max += std::cmp::max(0, x - height[i]);
        }

        max
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0042() {
        assert_eq!(6, Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
        assert_eq!(9, Solution::trap(vec![4,2,0,3,2,5]));
    }
}

