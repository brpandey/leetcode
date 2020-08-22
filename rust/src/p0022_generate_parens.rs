/*

22. Generate Parenthesis

 Given n pairs of parentheses, write a function to generate
all combinations of well-formed parentheses.

 */

//const TAB_SPACE: &str = "   ";

pub struct Solution {}

impl Solution {
    pub fn run(n: u32) -> Vec<String> {
        let mut acc: Vec<String> = Vec::new();
        Solution::generate(&mut acc, "", 0, 0, n, 0);

        acc
    }

    pub fn generate(list: &mut Vec<String>, acc: &str, left: u32, right: u32, max: u32, tabs: u32) {
//        println!("{}{}", TAB_SPACE.repeat(tabs as usize), acc);

        // If max is 3 we have three left parens and three right parens and hence 3 pairs
        if left == max && right == max {
            list.push(acc.to_string());
            return
        }

        // We haven't used up all the max left parens yet
        if left < max {
            Solution::generate(list, &[acc, "("].join(""), left + 1, right, max, tabs + 1);
        }

        // We have atleast one unclosed left parenthesis, so we can close it with a right paren
        // This check after the left parens insertion, ensures our parens are balanced, without this we could have ")))((("
        if right < left {
            Solution::generate(list, &[acc, ")"].join(""), left, right + 1, max, tabs + 1);
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

        assert_eq!(list, Solution::run(3));
    }
}


/*


(
  ((
    (((
      ((()
        ((())
          ((()))
    (()
      (()(
        (()()
          (()())
      (())
        (())(
          (())()
  ()
    ()(
      ()((
        ()(()
          ()(())
      ()()
        ()()(
          ()()()
*/
