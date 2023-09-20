/*
 * 875. Koko Eating Bananas
Medium

Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return the minimum integer k such that she can eat all the bananas within h hours.

 

Example 1:

Input: piles = [3,6,7,11], h = 8
Output: 4

Example 2:

Input: piles = [30,11,23,4,20], h = 5
Output: 30

Example 3:

Input: piles = [30,11,23,4,20], h = 6
Output: 23

 

Constraints:

    1 <= piles.length <= 104
    piles.length <= h <= 109
    1 <= piles[i] <= 109


 */

/*
   Given the 4 piles of bananas as denoted here -> [3,6,7,11]
   Try various bananas per hour rate or if greater than pile amount, bananas per pile
   Such that total hours does not exceed h or 8 hours

   Experiment with eating x bananas per hour, where x is between 1 and the largest pile value e.g. 11
   11 is the upper limit as we know that if k was 12, Koko will not eat more than a piles amount of bananas in an hour
   so in this case that would be 11

   While evaluating 1..11 bananas per hour, it doesn't need to be evaluated linearly, e.g. 1,2,3,4
   it can be evaluated via binary search so 1+11/2 or 6 first then either 3 or 9 next

   So a Koko k bananas/hour value of (11+1)/2 => 6 so that would be => [3/6, 6/6, 7/6, 11/6] or 1+1+2+2 => 6 hours
   6 hours is less than 8 hours, so next find an even smaller rate of k smaller than k that is still under 8 hours

   1+5 = 6 => 6/2 => 3 so that would be [3/3, 6/3, 7/3, 11/3] => 1+2+3+4 => 10 which is > 8
   4+5 = 9 => 9/2 => 4 so that would be [3/4, 6/4, 7/4, 11/4] => 1+2+2+3 => 8 which is == 8, found!

 */

pub struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_pile = 0;

        for i in 0..piles.len() {
            max_pile = std::cmp::max(max_pile, piles[i]);
        }

        let mut min_result = max_pile;
        let mut l = 0;
        let mut r = max_pile as usize;


        while l <= r {
            let k: usize = (l+r)/2; // middle value in eating speeds
            let mut hours: f32;
            let mut total_hours = 0;

            for i in 0..piles.len() { // calculate total hours it takes to eat bananas given k bananas/hour
                hours = piles[i] as f32/(k as f32);
                total_hours += hours.ceil() as i32;
            }

            if total_hours <= h { // shift binary search focus window to the smaller entries which consume slower, go left
                r = k - 1;
                min_result = std::cmp::min(k as i32, min_result);
            } else { // shift binary search focus window to larger entries that will consume more bananas quickly, go right
                l = k + 1;
            }
        }

        min_result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_875() {
        assert_eq!(4, Solution::min_eating_speed(vec![3,6,7,11], 8));
        assert_eq!(30, Solution::min_eating_speed(vec![30,11,23,4,20], 5));
        assert_eq!(23, Solution::min_eating_speed(vec![30,11,23,4,20], 6));
    }
}

