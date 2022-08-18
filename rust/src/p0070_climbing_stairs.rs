/*
 *
 * 70. Climbing Stairs
Easy

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

 

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps

Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step

 

Constraints:

    1 <= n <= 45


*/


/*
 * Thoughts
 *
 * Start from base case, and build upon subproblems to form dp list
 * Essentially label each stair and think about paths
 *
 *
 *
 *                  []
 *               [] []
 *            [] [] []
 *         [] [] [] []
 *      [] [] [] [] []
 *
 *   0  1  2  3  4  5
 *   A  B  C  D  E  F
 *
 *   Starting from :
 *   Step 5 F, base case, so put 1 as dp[5] = 1, F
 *   Step 4 E, 1 step, so dp[4] = 1, as 2 steps would be past top, EF
 *   Step 3 D, 1 step to E, e.g. DE + E's combinations which is EF => (DE, EF)
 *           and we can do 2 steps to F, or DF + F's combinations which is F => (DF) 
 *           => Hence 2 combinations
 *
 *   Step 2 C, we can do 1 step to D, e.g. CD + D's combinations (CD, DE, EF) or (CD, DF)
 *           and we can do 2 steps to E, or CE + E's combinations (CE, EF) 
 *           => Hence 3 combinations
 *   Step 1 B, we can do 1 step to C, e.g. BC + C's combinations (3) and do two steps to D,
 *           e.g. BD + D's combinations (2)
 *           => Hence 5 combinations
 *   Step 0 A, we can do 1 step to B, e.g. AB + B's combinations (5) and two steps to C,
 *           e.g. AC + C's combinations (3)
 *           => Hence 8 combinations
 *
 *  Thus dp[i] = dp[i+1] // 1 step + dp[i+2] // 2 step
 *
 *       0  1  2  3  4  5
 *  dp:  8  5  3  2  1  1
 *
 *  or Instead of creating a list,
 *
 *  Just keep adding the 1 step value and the 2 step value for the current value
 */

pub struct Solution {}

impl Solution {
    #[allow(unused_assignments)]
    pub fn climb_stairs(n: i32) -> i32 {

        let (mut onestep, mut twostep, mut sum) = (1, 0, 0);     
    
        // assume starting on step 4 if n = 5, as 1 step from 4 is 5 which is 1 path
        // don't need rev, just for illustrative purposes given figure above
        for _i in (0..n as usize).rev() {  
            sum = onestep + twostep;
            twostep = onestep; // shift over as we're moving to the left
            onestep = sum // shift over as we're moving to the left
        }

        onestep
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0070() {
        assert_eq!(8, Solution::climb_stairs(5));
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
    }
}

