/*
518. Coin Change II
    Medium

    You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

    Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.

    You may assume that you have an infinite number of each kind of coin.

    The answer is guaranteed to fit into a signed 32-bit integer.

    Example 1:

Input: amount = 5, coins = [1,2,5]
    Output: 4
    Explanation: there are four ways to make up the amount:
5=5
    5=2+2+1
    5=2+1+1+1
    5=1+1+1+1+1

    Example 2:

Input: amount = 3, coins = [2]
    Output: 0
    Explanation: the amount of 3 cannot be made up just with coins of 2.

    Example 3:

Input: amount = 10, coins = [10]
    Output: 1

    Constraints:

1 <= coins.length <= 300
   1 <= coins[i] <= 5000
      All the values of coins are unique.
      0 <= amount <= 5000
*/


use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amt = amount as usize;
        let cols: HashMap<usize, usize> = (0..=amt).rev().zip(0..=amt).into_iter().collect(); // columns, map amount_col to actual_col 5,4,3,2,1,0 -> 0,1,2,3,4,5

        // initialize a table matrix used for dynamic programming
        // basically, take the sum of the entry to the bottom and entry to the coin value spaces right
        let coins_len = coins.len();
        let rows = vec![0; amt+1]; // cols are: amount..0, or 5,4,3, ..,0, rows are: coins e.g. 1,2,5
        let mut dp = vec![rows; coins_len+1]; // create coins_len rows if three coins, then four rows (last row a stub row)

        for r in 0..coins_len {
            let c = Self::col(0, &cols);
            dp[r][c] = 1; // initialize the base cases where the 0th col values are all 0
        }

        // start populating the dp matrix, start from bottom right, e.g. last row first col and then
        // e.g. given e.g. cols 5,4,3,2,1,0 and rows 1,2,5
        // so start at biggest row smallest col, or (5,0)

        for r in (0..coins_len).rev() {
            for amt_col in 1..=amt { // e.g. start from the right, go to the left 0,1,2,3,4,5 amount col
                let c = Self::col(amt_col, &cols); // map human readable column value amount_col to actual indexable column value
                dp[r][c] += dp[r+1][c]; // Add value from row directly below (this is where stub row comes in handy for first iterations)

                let current_coin = coins[r] as usize;
                if amt_col >= current_coin { // ensure that given the current coin value,
                                             // we can subtract it from current amount col in dp matrix (otherwise we've left the matrix edges)
                    dp[r][c] += dp[r][c + current_coin] // same row just add contribution from value coins[r] to the right
                }
            }
        }

        //        println!("table is {:#?}", &dp);

        dp[0][0] // return value from top left or 0th row, and 0th col
    }

    // maps cols amount...0 to 0..amount, so e.g. 5,4,3,2,1,0 -> 0,1,2,3,4,5
    // doing this because a table  5,4,3,2,1,0
    //                           1 E
    //                           2
    //                           5           S   is easier to read
    pub fn col(inverted: usize, cols: &HashMap<usize, usize>) -> usize {
        *cols.get(&inverted).unwrap()
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0518() {
//        Input: amount = 5, coins = [1,2,5]
//            Output: 4
        assert_eq!(4, Solution::change(5, vec![1,2,5]));
//        Input: amount = 3, coins = [2]
//            Output: 0
        assert_eq!(0, Solution::change(3, vec![2]));
//        Input: amount = 10, coins = [10]
//            Output: 1
        assert_eq!(1, Solution::change(10, vec![10]));

    }
}

