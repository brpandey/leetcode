/*
 * 224. Basic Calculator
Hard

Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.

Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

Example 1:

Input: s = "1 + 1"
Output: 2

Example 2:

Input: s = " 2-1 + 2 "
Output: 3

Example 3:

Input: s = "(1+(4+5+2)-3)+(6+8)"
Output: 23

Constraints:
    1 <= s.length <= 3 * 105
    s consists of digits, '+', '-', '(', ')', and ' '.
    s represents a valid expression.
    '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
    '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
    There will be no two consecutive operators in the input.
    Every number and running calculation will fit in a signed 32-bit integer.
 */

pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<(i32, i32)> = vec![];
        let (mut sum, mut num, mut sign): (i32, i32, i32) = (0, 0, 1);
        let mut prev_sum;

        for ch in s.chars() {
            if ch >= '0' && ch <= '9' {
                num = num * 10 + ch as i32 - '0' as i32;
            }

            // when + or - is encountered, accumulate everything up to this point
            // setting sign and resetting num
            // For parens, use a stack and push on current sum and sign,
            // and resetting sum, sign and num
            match ch {
                '+' => {
                    sum += sign*num;
                    sign = 1;
                    num = 0;
                },
                '-' => {
                    sum += sign*num;
                    sign = -1;
                    num = 0;
                },
                ' ' => {
                    continue
                },
                '(' => {
                    stack.push((sign,sum));
                    sum = 0;
                    num = 0;
                    sign = 1;
                },
                ')' => {
                    sum += sign*num;
                    (sign, prev_sum) = stack.pop().unwrap();
                    sum = prev_sum + sign*sum;
                    sign = 1;
                    num = 0;
                },
                _ => (),
            }
        }

        // ensure we grab last number if not present just adding to 0
        sum += sign*num;
        sum
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0224() {
        assert_eq!(2, Solution::calculate("1 + 1".to_string()));
        assert_eq!(3, Solution::calculate(" 2-1 + 2 ".to_string()));
        assert_eq!(23, Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
        assert_eq!(-27, Solution::calculate("(1-(4+5+2)-3)-(6+8)".to_string()));
    }
}

