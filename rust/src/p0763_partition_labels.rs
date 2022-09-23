/*
 * 763. Partition Labels
Medium

You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.

Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.

Return a list of integers representing the size of these parts.

 

Example 1:

Input: s = "ababcbacadefegdehijhklij"
Output: [9,7,8]
Explanation:
The partition is "ababcbaca", "defegde", "hijhklij".
This is a partition so that each letter appears in at most one part.
A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.

Example 2:

Input: s = "eccbbbbdec"
Output: [10]

 */

/*
 * input string: "ababcbacadefegdehijhklij"
 * 
 * start from right to left, if character key not present add index to map
 *
 * then traverse through string again, with two markers window_size, window_end
 * if we pass window end as string is iterated through, than add current window to result
 * and reset window
 */

use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut map = HashMap::with_capacity(26);
        let mut result = vec![];

        for (i, b) in s.bytes().enumerate().rev() {
            map.entry(b).or_insert(i);
        }

        let mut window_size = 0;
        let mut window_end = 0;

        for (i, b) in s.bytes().enumerate() {
            // grab character last index
            let last = *map.get(&b).unwrap();

            // if last index is past window end, update
            // window end (not done with current window yet)
            if last > window_end {
                window_end = last
            }

            window_size += 1;
            
            if window_end == i {
                result.push(window_size);
                window_size = 0;
                window_end = 0;
            }


        }

        result
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0763() {
        assert_eq!(vec![9,7,8], Solution::partition_labels("ababcbacadefegdehijhklij".to_string()));
        assert_eq!(vec![10], Solution::partition_labels("eccbbbbdec".to_string()));
    }
}
