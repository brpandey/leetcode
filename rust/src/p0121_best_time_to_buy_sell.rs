/*
121. Best Time to Buy and Sell Stock
Easy

Say you have an array for which the ith element is the price of a given stock on day i.

If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.

Note that you cannot sell a stock before you buy one.

Example 1:

Input: [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
Not 7-1 = 6, as selling price needs to be larger than buying price.

Example 2:

Input: [7,6,4,3,1]
Output: 0
Explanation: In this case, no transaction is done, i.e. max profit = 0.

*/

use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn run(nums: &[i32]) -> i32 {
        if nums.len() < 1 { return 0 }
        let (mut sell, mut min_buy);
        let mut profit_max = 0;

        min_buy = nums[0];

        // 7,1,5,3,6,4, first iter sell = 1, min_buy 7, second iter sell = 5, min_buy 1
        for i in 1..nums.len() {
            sell = nums[i]; // enumerate through the list with sell being current element
            profit_max = cmp::max(sell - min_buy, profit_max); // keep highest
            min_buy = cmp::min(sell, min_buy) // keep lowest buy stock
        }

        profit_max
    }

    // uses fold
    pub fn run1(nums: &[i32]) -> i32 {
        let acc = nums
            .iter()
            .skip(1) // we need to start at the second item in the list to be able to compare
            .fold((0, nums[0]), |(mut profit_max, mut buy), &sell| {
                profit_max = cmp::max(sell - buy, profit_max);
                // if buy is greater than sell, change buy value to sell value (we want to buy low)
                // e.g. buy is 7 and sell is 1 => change to => buy is 1
                if sell - buy < 0 { buy = sell };

                (profit_max, buy)
            });

        acc.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0121() {
        assert_eq!(5, Solution::run(&[7,1,5,3,6,4]));
        assert_eq!(0, Solution::run(&[7,6,4,3,1]));
        assert_eq!(655, Solution::run(&[100, 180, 260, 310, 40, 535, 695]));

        // nums are [7, 1, 5, 3, 6, 4]
        // sell is 1, buy is 7, profit_max is +0
        // sell is 5, buy is 1, profit_max is +4
        // sell is 3, buy is 1, profit_max is +4
        // sell is 6, buy is 1, profit_max is +5
        // sell is 4, buy is 1, profit_max is +5

        // nums are [7, 6, 4, 3, 1]
        // sell is 6, buy is 7, profit_max is +0
        // sell is 4, buy is 6, profit_max is +0
        // sell is 3, buy is 4, profit_max is +0
        // sell is 1, buy is 3, profit_max is +0

        // nums are [100, 180, 260, 310, 40, 535, 695]
        // sell is 180, buy is 100, profit_max is +80
        // sell is 260, buy is 100, profit_max is +160
        // sell is 310, buy is 100, profit_max is +210
        // sell is 40, buy is 100, profit_max is +210
        // sell is 535, buy is 40, profit_max is +495
        // sell is 695, buy is 40, profit_max is +655

    }
}
