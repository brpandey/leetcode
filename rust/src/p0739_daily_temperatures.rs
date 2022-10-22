/*
 * 739. Daily Temperatures
Medium

Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. If there is no future day for which this is possible, keep answer[i] == 0 instead.

 

Example 1:

Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]

Example 2:

Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]

Example 3:

Input: temperatures = [30,60,90]
Output: [1,1,0]

 

Constraints:

    1 <= temperatures.length <= 105
    30 <= temperatures[i] <= 100


 */

/*
 * Strategy
 *
 * Given input [73,74,75,71,69,72,76,73]
 * Use a stack                                         
 * Pop stack if stack head is < new element and diff new element index with stack head index
 *
 *     |---stack top
 * [X, Y]
 *
 * Stack history
 * [(73,0)] -> [(74,1)] -> [(75,2)] -> [(75,2),(71,3)] -> [(75,2),(71,3),(69,4)] -> [(75,2),(72,5)] -> [(76,6)] -> [(76,6),(73,7)]
 * (Note: stack is decreasing from bottom to top)
 *
 *    0   1   2   3   4   5  6  7
 * [1-0,2-1,6-2,5-3,5-4,6-5, 0, 0]
 *
 * [1,1,4,2,1,1,0,0]
 */

pub struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut result: Vec<i32> = vec![0; temperatures.len()];

        // Scan through temperature values, while value is greater than
        // stack top, pop stack, and update result vector accordingly with # days
        for i in 0..temperatures.len() {
            let t = temperatures[i];
            while !stack.is_empty() && stack[stack.len() - 1].0 < t {
                let (_v, v_index) = stack.pop().unwrap();
                result[v_index] = (i - v_index) as i32;
            }

            stack.push((t,i));
        }

        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0739() {
        assert_eq!(vec![1,1,4,2,1,1,0,0], Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]));
        assert_eq!(vec![1,1,1,0], Solution::daily_temperatures(vec![30,40,50,60]));
        assert_eq!(vec![1,1,0], Solution::daily_temperatures(vec![30,60,90]));
    }
}

