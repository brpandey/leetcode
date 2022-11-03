/*
8. String to Integer (atoi)
Medium

Implement atoi which converts a string to an integer.

The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.

The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.

If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.

If no valid conversion could be performed, a zero value is returned.

Note:

    Only the space character ' ' is considered as whitespace character.
    Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231) is returned.

Example 1:

Input: "42"
Output: 42

Example 2:

Input: "   -42"
Output: -42
Explanation: The first non-whitespace character is '-', which is the minus sign.
             Then take as many numerical digits as possible, which gets 42.

Example 3:

Input: "4193 with words"
Output: 4193
Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.

Example 4:

Input: "words and 987"
Output: 0
Explanation: The first non-whitespace character is 'w', which is not a numerical 
             digit or a +/- sign. Therefore no valid conversion could be performed.

Example 5:

Input: "-91283472332"
Output: -2147483648
Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
             Thefore INT_MIN (−231) is returned.


*/


use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn atoi(val: &str) -> i32 {
        let mut iter = val.chars().peekable();
        let mut acc: i32 = 0;
        let mut sign: i32 = 1;
        let nums: HashSet<&char> = ['0','1','2','3','4','5','6','7','8','9'].iter().collect();

        // Consume leading whitespace
        while iter.peek() == Some(&' ') {iter.next();}

        // Consume any optional sign info
        match iter.peek() {
            Some('-') => {sign = -1;iter.next();},
            Some('+') => {iter.next();}
            _ => (),
        }

        // Grab numbers
        while let Some(ch) = iter.next() {
            if nums.contains(&ch) {
                acc = acc * 10 + (ch as u32 - '0' as u32) as i32
            }
            else {
                break
            }
        }
        acc * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0008() {
        assert_eq!(42, Solution::atoi("42"));
        assert_eq!(-42, Solution::atoi("    -42"));
        assert_eq!(4193, Solution::atoi("4193 with words"));
        assert_eq!(0, Solution::atoi("words and 987"));
    }
}
