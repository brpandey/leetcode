/*
787. Cheapest Flights Within K Stops
Medium

There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.

You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.


Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
Output: 700
Explanation:
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 3 is marked in red and has cost 100 + 600 = 700.
Note that the path through cities [0,1,2,3] is cheaper but is invalid because it uses 2 stops.

Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 1
Output: 200
Explanation:
The graph is shown above.
The optimal path with at most 1 stop from city 0 to 2 is marked in red and has cost 100 + 100 = 200.

Input: n = 3, flights = [[0,1,100],[1,2,100],[0,2,500]], src = 0, dst = 2, k = 0
Output: 500
Explanation:
The graph is shown above.
The optimal path with no stops from city 0 to 2 is marked in red and has cost 500.
*/

pub const INFINITY: i32 = i32::MAX;

pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut prices = vec![INFINITY; n as usize];
        prices[src as usize] = 0;

        let mut new_prices: Vec<i32>;

        // use bellman ford algorithm
        // update destination prices to new_prices list given values from original prices
        // upon each k iteration set new_prices to prices

        for _i in 0..k+1 {

            new_prices = prices.clone();

            // iterate through all edges in flights directed graph (vec of vec)
            for edge in &flights {
                let (s, d, p) = (edge[0] as usize, edge[1] as usize, edge[2]);

                // lookup the source in the prices vec, if it is "infinity" ignore
                // because we aren't able to optimize for its destination if source is infinity still
                if prices[s] == INFINITY { continue }

                if prices[d] > prices[s] + p {
                    new_prices[d] = prices[s] + p;
                }
            }

            prices = new_prices;
        }

        if prices[dst as usize] == INFINITY { // meaning there's no path to dest, return -1
            -1
        } else {
            prices[dst as usize]
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0787() {
        assert_eq!(700, Solution::find_cheapest_price(4, vec![vec![0,1,100], vec![1,2,100], vec![2,0,100], vec![1,3,600], vec![2,3,200]], 0, 3, 1));
        assert_eq!(200, Solution::find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]], 0, 2, 1));
        assert_eq!(500, Solution::find_cheapest_price(3, vec![vec![0,1,100], vec![1,2,100], vec![0,2,500]], 0, 2, 0));
    }
}

