/*
20. Valid Parentheses
Easy

Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.

Note that an empty string is also considered valid.

Example 1:

Input: "()"
Output: true

Example 2:

Input: "()[]{}"
Output: true

Example 3:

Input: "(]"
Output: false

Example 4:

Input: "([)]"
Output: false

Example 5:

Input: "{[]}"
Output: true
*/
pub struct Solution {}

impl Solution {
    pub fn run(input: &str) -> bool {

        let mut stack: Vec<char> = Vec::new();

        for c in input.chars() {
            match c {
                '(' => {
                    stack.push(')');
                },
                '{' => {
                    stack.push('}');
                },
                '[' => {
                    stack.push(']');
                },
                ')' => {
                    if let Some(')') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                },
                '}' => {
                    if let Some('}') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                },
                ']' => {
                    if let Some(']') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0020() {
        assert_eq!(true, Solution::run("()"));
        assert_eq!(true, Solution::run("()[]{}"));
        assert_eq!(false, Solution::run("(]"));
        assert_eq!(false, Solution::run("(])]"));
        assert_eq!(true, Solution::run("{[]}"));
    }
}
