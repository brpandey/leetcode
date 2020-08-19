/*
11. Container With Most Water
Medium

Given n non-negative integers a1, a2, ..., an , where each represents a point at
coordinate (i, ai). n vertical lines are drawn such that the two endpoints of
line i is at (i, ai) and (i, 0). Find two lines, which together with x-axis
forms a container, such that the container contains the most water.

Note: You may not slant the container and n is at least 2.

The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this
case, the max area of water (blue section) the container can contain is 49.



Example:

Input: [1,8,6,2,5,4,8,3,7]
Output: 49
*/
use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn container_most_water(numbers: &[u32]) -> u32 {
        let (mut i, mut j, mut max_area) =
            (0 as usize, (numbers.len()-1) as usize, 0 as usize);

        let mut area: usize;

        while i < j {
            // Compute rectangle area using numbers value as y and indice delta as x
            area = (j-i) * cmp::min(numbers[i], numbers[j]) as usize;
            max_area = cmp::max(area, max_area);

            // We shrink the window space, rather than using a brute force solution
            // of two for loops
            if numbers[i] > numbers[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }

        max_area as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0011(){
        assert_eq!(49, Solution::container_most_water(&[1,8,6,2,5,4,8,3,7]));
    }
}
