/*

22. Generate Parenthesis

 Given n pairs of parentheses, write a function to generate
all combinations of well-formed parentheses.

 */

pub struct Solution {}

impl Solution {
    pub fn run(n: u32) -> Vec<String> {
        let mut acc: Vec<String> = Vec::new();
        Solution::generate(&mut acc, "", 0, 0, n);

        acc
    }

    pub fn generate(list: &mut Vec<String>, acc: &str, open: u32, close: u32, max: u32) {
        if acc.len() == (max * 2) as usize {
            list.push(acc.to_string());
            return
        }

        if open < max {
            Solution::generate(list, &[acc, "("].join(""), open + 1, close, max);
        }

        if close < open {
            Solution::generate(list, &[acc, ")"].join(""), open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0022(){

        let list = vec![
            "((()))",
            "(()())",
            "(())()",
            "()(())",
            "()()()"
        ];

//        let list = vec!["((()))"];
        assert_eq!(list, Solution::run(3));
    }
}
