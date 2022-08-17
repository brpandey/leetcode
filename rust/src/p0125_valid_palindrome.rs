/*
 *
 * 125. Valid Palindrome
Easy

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

 

Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.

Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.

Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.

 

Constraints:

    1 <= s.length <= 2 * 105
    s consists only of printable ASCII characters.

*/

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v = s.chars().collect::<Vec<char>>();
        let (mut l, mut r) = (0, v.len()-1);

        // use a sliding window approach where l and r converge in on each other
        // as this is done ensure that ascii chars are the same,
        // skipping over non-alpha numeric characters
        while l < r {
            if !Solution::is_alphanumeric(v[l]) {
                l += 1;
                continue
            }

            if !Solution::is_alphanumeric(v[r]) {
                r -= 1;
                continue;
            }

            if v[l] != v[r] { return false }

            l += 1;
            r -= 1;
        }

        true
    }

    pub fn is_alphanumeric(c: char) -> bool {
        if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
            return true
        } else {
            return false
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0125() {
        assert_eq!(false, Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
        assert_eq!(true, Solution::is_palindrome(" ".to_string()));
    }
}

