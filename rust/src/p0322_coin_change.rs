// 322. Coin Change
//     Medium

//     You are given coins of different denominations and a total amount of money amount. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

//     Example 1:

// Input: coins = [1, 2, 5], amount = 11
//     Output: 3 
//     Explanation: 11 = 5 + 5 + 1

//     Example 2:

// Input: coins = [2], amount = 3
//     Output: -1

//     Note:
// You may assume that you have an infinite number of each kind of coin.


// macro_rules! hashset(
//     { $($key:expr ),+ } => {
//         {
//             let mut m = ::std::collections::HashSet::new();
//             $(
//                 m.insert($key);
//             )+
//                 m
//         }
//     };
// );


use std::cmp;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    // The base case for this problem MC[amount], where amount = 0, is zero coins, MC(0) = 0
    // The task essentially is to populate the recurrence relation MC(n) = MC(n-1) + 1
    // Given an arbitrary amount value say amount = 8, we can break the problem into subproblems
    // given m number of coins at our disposal (1,2,5), we can break this down into m subproblems
    // min(MC(8-1), MC(8-2), MC(8-5)) + 1 = min(MC(7), MC(6), MC(3)) + 1
    // Each of these subtractors (1, 2, 5) are items in our coins set hence the +1 to indicate that we use one of them
    // We push the problem to then the smaller subproblems of MC(7), MC(6), MC(3) down to MC(0)

    // For example, MC(3) in this case will be defined similarily in terms of MC(3) = min(MC(3-1), MC(3-2), MC(3-5)) + 1
    // Ignoring negative values we have = min(MC(2), MC(1)) + 1

    pub fn run(amount: i8, coins: &HashSet<i8>) -> i8 {
        // Create a vector from 0...amount storing the optimized min count amounts for that value wrapped in an Option type
        // For example min_coins[4] = is the minimum number of coins which total $4 = 2 (essentially 2 $2 coins)
        let mut min_coins: Vec<Option<i8>> = vec![None; amount as usize +1];
        min_coins[0] = Some(0); // base case

        // When doing min value comparison, if we don't have a value,
        // use the highest value to avoid rejecting a low legitimate number
        // We only use default when we are doing the min comparison (it shouldn't be the value in the actual vector)
        let default = i8::MAX;

        for sub_amount in 1..=amount {
            for &single_coin in coins.iter() {
                // For each coin store the optimal min_coins in min_coins[a] until we have iterated through all the coins
                // We're only interested in coins that are less than or equal to the current amount

                // Note if we try to add a None value we use either default or default-1 as the actual number
                if single_coin <= sub_amount { // If the single coin is 5 and the current amount is 3, we skip
                    let a = sub_amount as usize;
                    let coin = single_coin as usize;

                    // Here min_coins[a] is being used as a min accumulator until we're done, consider this case where a is 5
                    // single coin is 1, a is 5, min is 3,
                    // single coin is 2, a is 5, min is 3,
                    // single coin is 5, a is 5, min is 1,

                    // In more verbose DP programming solutions, each of these values is stored separately which is overkill
                    // Instead we just overwrite the min_accumulator with the new lowest min
                    let min_acc = min_coins[a].unwrap_or(default);
                    let compare_plus_one = min_coins[a - coin].unwrap_or(default-1).checked_add(1).unwrap_or(default);

                    // Put it back to an option type
                    match cmp::min(min_acc, compare_plus_one) {
                        i8::MAX => min_coins[a] = None,
                        min => min_coins[a] = Some(min),
                    }
                }
            }
        }

        if let Some(value) = min_coins[amount as usize] { value } else { -1 }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use maplit::hashset;

    #[test]
    fn test_0322() {
        assert_eq!(3, Solution::run(11, &hashset!{1, 2, 5}));
        assert_eq!(-1, Solution::run(3, &hashset!{2}));
    }
}


