/*
 * 338. Counting Bits
Easy

Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

Example 1:

Input: n = 2
Output: [0,1,1]
Explanation:
0 --> 0
1 --> 1
2 --> 10

Example 2:

Input: n = 5
Output: [0,1,1,2,1,2]
Explanation:
0 --> 0
1 --> 1
2 --> 10
3 --> 11
4 --> 100
5 --> 101

Constraints:

    0 <= n <= 105

Follow up:

    It is very easy to come up with a solution with a runtime of O(n log n). Can you do it in linear time O(n) and possibly in a single pass?
    Can you do it without using any built-in function (i.e., like __builtin_popcount in C++)?

 */


/* num   binary     num of binary 1's
 *   0:  0 0 0 0  = 0
 *   1:  0 0 0 1  = 1 or 1 + dp[1-1]
 *   2:  0 0 1 0  = 1 or 1 + dp[2-2]
 *   3:  0 0 1 1  = 2 or 1 + dp[3-2]
 *
 * notice 4 is the same as 0, but with a 1 attached
 *   4:  0 1 0 0  = 1 or 1 + dp[4-4]
 *   5:  0 1 0 1  = 2 or 1 + dp[5-4]
 *   6:  0 1 1 0  = 2 or 1 + dp[6-4]
 *   7:  0 1 1 1  = 3 or 1 + dp[7-4]
 *
 * notice 8 is the same as 0, but with a 1 attached
 *   8:  1 0 0 0  = 1 or 1 + dp[8-8]
 *   9:  1 0 0 1  = 2 or 1 + dp[9-8]
 *  10:  1 0 1 0  = 2 or 1 + dp[10-8]
 *
 */

pub struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // dp is initialized to all zeroes
        let mut dp: Vec<i32> = vec![0; n as usize +1];
        let mut offset = 1;

        for i in 1..(n+1) as usize {
            if i == offset*2 {
                offset = i; // e.g. 2,4,8,16...
            }

            // Use the value previously computed numbers
            dp[i] = 1 + dp[i-offset]; 
        }

        dp
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0338() {
        assert_eq!(vec![0,1,1], Solution::count_bits(2));
        assert_eq!(vec![0,1,1,2,1,2], Solution::count_bits(5));
    }
}

