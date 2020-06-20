use std::cmp;

struct Solution{}

impl Solution {

    fn partition(nums1: &[i32], nums2: &[i32], low: usize, high: usize, m: usize, n: usize) -> f32 {
        let p_x = (m + n) / 2;
        let p_y = (m + n + 1)/2 - p_x;

        let max_left_x = if p_x > 0 {nums1[p_x-1]} else {i32::MIN};
        let min_right_x = if p_x == m {i32::MAX} else {nums1[p_x]};

        let max_left_y = if p_y > 0 {nums2[p_y-1]} else {i32::MIN};
        let min_right_y = if p_y == n {i32::MAX} else {nums2[p_y]};

        println!("p_x is {:?}", p_x);
        println!("p_y is {:?}", p_y);

        if max_left_x <= min_right_y && min_right_x >= max_left_y {
            match (m + n) % 2 == 0 {
                true => (cmp::max(max_left_x, max_left_y) + cmp::min(min_right_x, min_right_y)) as f32 / 2.0,
                false => cmp::max(max_left_x, max_left_y) as f32,
            }
            //check if even avg(sum(left))
        } else if max_left_x > min_right_y {
            //Go left to pick up smaller values
            Solution::partition(nums1, nums2, low, p_x-1, m, n)
        } else {
            //Go right to pick up bigger values
            Solution::partition(nums1, nums2, p_x+1, high, m, n)
        }
    }

    fn median_two_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f32 {
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
//        assert_eq!(2.0, Solution::median_two_sorted_arrays(&[1,3], &[2]));
//        assert_eq!(2.5, Solution::median_two_sorted_arrays(&[1,2], &[3, 40]));
        assert_eq!(4.0, Solution::median_two_sorted_arrays(&[1, 2, 3, 4, 7], &[0, 5, 6, 9]));
//        assert_eq!(4.5, Solution::median_two_sorted_arrays(&[1, 2, 3, 4, 7], &[0, 5, 6, 9, 100]));
    }
}
