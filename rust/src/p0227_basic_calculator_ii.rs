/*
 * 227. Basic Calculator II
Medium

Given a string s which represents an expression, evaluate this expression and return its value. 

The integer division should truncate toward zero.

You may assume that the given expression is always valid. All intermediate results will be in the range of [-231, 231 - 1].

Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

Example 1:

Input: s = "3+2*2"
Output: 7

Example 2:

Input: s = " 3/2 "
Output: 1

Example 3:

Input: s = " 3+5 / 2 "
Output: 5

Constraints:

    1 <= s.length <= 3 * 105
    s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
    s represents a valid expression.
    All the integers in the expression are non-negative integers in the range [0, 231 - 1].
    The answer is guaranteed to fit in a 32-bit integer.
 */

/*
 * Strategy
 * Push the first number we end up parsing on the stack
 * If its a space skip to next iteration unless we're at the end
 * Only store numbers on stack and if we have * or / compute it right there
 * With any remaining numbers on the stack just add
 * Minus e.g. x - y is x + (y * -1)
 *
 * The trick here is use the operator information for the next pass,
 * after we've already pushed the first number and have just parsed the second number
 */

pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut last: i32 = 0;
        let mut stack: Vec<i32> = vec![];
        let mut operator: char = '!';
        let mut digit;

        let list: Vec<char> = s.chars().collect();
        let len = list.len();

        for (i, c) in list.iter().enumerate() {
            digit = *c >= '0' && *c <= '9';

            // only skip for whitespace if not last char
            if *c == ' ' && i + 1 != len {
                continue
            }

            // accumulate number if multi-digit
            if digit {
                last = last * 10 + (*c as i32 - '0' as i32);
            } 

            // push and pop from stack
            // for mult and div resolve immediately since higher precedence
            if !digit || i + 1 == len {
                match operator {
                    '+' | '!' => {
                        stack.push(last);
                    },
                    '-' => {
                        stack.push(last * -1);
                    },
                    '*' => {
                        let temp = stack.pop().unwrap();
                        stack.push(temp*last);
                    },
                    '/' => {
                        let temp = stack.pop().unwrap();
                        stack.push(temp/last);
                    },
                    _ => (),
                }

                // trick is to have a local variable 'stack' operator
                // to capture what the operation is, used once we parse the next
                // number
                operator = *c;
                // if we've seen an operator that means the last number has already
                // been pushed to stack and we reset it to create space for new one
                last = 0;
            }
        }

        let mut sum = 0;

        // commutatively sum reduce the remaining of the stack
        while !stack.is_empty() {
            sum += stack.pop().unwrap();
        }

        sum
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0227() {
        assert_eq!(7, Solution::calculate("3+2*2".to_string()));
        assert_eq!(1, Solution::calculate(" 3 / 2 ".to_string()));
        assert_eq!(5, Solution::calculate(" 3 + 5 / 2 ".to_string()));
    }
}

