/*
 *
 * 91. Decode Ways
Medium

A message containing letters from A-Z can be encoded into numbers using the following mapping:

'A' -> "1"
'B' -> "2"
...
'Z' -> "26"

To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:

    "AAJF" with the grouping (1 1 10 6)
    "KJF" with the grouping (11 10 6)

Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".

Given a string s containing only digits, return the number of ways to decode it.

The test cases are generated so that the answer fits in a 32-bit integer.

 

Example 1:

Input: s = "12"
Output: 2
Explanation: "12" could be decoded as "AB" (1 2) or "L" (12).

Example 2:

Input: s = "226"
Output: 3
Explanation: "226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).

Example 3:

Input: s = "06"
Output: 0
Explanation: "06" cannot be mapped to "F" because of the leading zero ("6" is different from "06").

 
Constraints:

    1 <= s.length <= 100
    s contains only digits and may contain leading zero(s).


 *
 */

/*
  Given a sample input array of 14216

         input   dp  dp_index

                  1  0
            1     1  1
            4     2  2  (1,4 = AD) or (14=N) = 2 ways to decode
            2     2  3  (1,4,2 = ADB) or (14,2 = NB) = 2  Since 42 > 26 just do i-1 or 2 (the previous result)

            1     4  4  (1,4,2,[1] = ADBA) or (14,2,[1] = NBA) -- this is dp[i-1] and 1 which is 2,
                        or (1,4,[21]=ADU) or (14,[21]=NU)  Since 21 < 26 -- this is dp[i-2] and 21 which is 2 as well

            6     6   5  The alg says take from [i-1] and [i-2] if 16 <= 26, so this is 4 + 2 = 6 let's verify
                        1,4,2,1,[6]  (essentially just adding 6 to the previous 4 items calculated at i-1)
                        14,2,1,[6]
                        1,4,21,[6]
                        14,21,[6]

                        1,4,2,[16] (essentially just adding 16 to the previous previous 2 items calculated at i-2)
                        14,2,[16]

  input array  [1, 4, 2, 1, 6 ]

*/

pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // Use a dynamic programming list to track the
        // number of ways to decode a string for each length of String (0,1,2...)

        // Base cases are size 0 and 1
        let mut dp: Vec<i32> = vec![0; s.len()+1];
        dp[0] = 1;  // base to support algorithm - string length of size 0 doesn't mean anything intrinsically
        dp[1] = if &s[0..1] == "0" { 0 } else { 1 }; // if string contains a 0 as first char there are zero ways to decode
                                               // otherwise for a string of length 1 ('A', 'B', 'C', .. ) there is only
                                               // 1 way
        // Handle recursive cases
        for i in 2..=s.len() {
            // grab string that constitutes 1 digit
            // e.g. "12" are two numbers 1 and number 2, here just grab number 2
            let onedigit = *&s[i-1..i].parse::<i32>().unwrap();

            // e.g. "12" is the number 12
            let twodigit = *&s[i-2..i].parse::<i32>().unwrap();

            // One digit is within 1..=9
            if onedigit >= 1 {
                dp[i] = dp[i] + dp[i-1];
            }

            // Two digits are within 10..=26
            // if greater than 26 than we ignore and just take contribution from single digit
            if twodigit >= 10 && twodigit <= 26 {
                dp[i] = dp[i] + dp[i-2];
            }
        }

        dp[s.len()]
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0091() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
        assert_eq!(3, Solution::num_decodings("226".to_string()));
        assert_eq!(0, Solution::num_decodings("06".to_string()));
        assert_eq!(6, Solution::num_decodings("14216".to_string()));
    }

    /*
    running 1 test
        &dp = [
            1,
            1,
            2,
        ]
        &dp = [
            1,
            1,
            2,
            3,
        ]
        &dp = [
            1,
            0,
            0,
        ]
        &dp = [
            1,
            1,
            2,
            2,
            4,
            6,
        ]
    */
}

