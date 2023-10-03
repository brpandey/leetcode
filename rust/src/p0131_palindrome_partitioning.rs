
/*
131. Palindrome Partitioning

Given a string s, partition s such that every substring of the partition is a palindrome.
Return all possible palindrome partitioning of s.

Example 1:

Input: s = "aab"
Output: [["a","a","b"],["aa","b"]]
Example 2:

Input: s = "a"
Output: [["a"]]
*/

pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut output = vec![];
        let mut current = vec![];

        Self::dfs(0, s.as_bytes(), &mut current, &mut output);
        output
    }

    pub fn dfs(l: usize, s: &[u8], cur: &mut Vec<String>, out: &mut Vec<Vec<String>>) {
        if l >= s.len() { // e.g. for the first dfs path run cur is vec!["a", "a", b"]
            out.push(cur.clone());
        }

        for r in l..s.len() { // Notice r is not fixed!
            if Self::is_palindrome(s, l, r) {
                // so for "aab", first palindrome is "a" remainder "ab", then on next iteration is "aa", remainder "b""
                cur.push(String::from_utf8_lossy(&s[l..r+1]).into_owned()); // add partition string to vec
                Self::dfs(r+1, s, cur, out);
                cur.pop();
            }
        }
    }

    pub fn is_palindrome(s: &[u8], mut l: usize, mut r: usize) -> bool {
        if l >= s.len() || r >= s.len() { return false }

        while l < r {
            if s[l] != s[r] {
                return false
            }

            l += 1;
            r -= 1;
        }

        true
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    // cargo test -- --color always --nocapture p0XXX::tests::test_0XXX

    #[test]
    pub fn test_0131() {
        let check = vec![vec!["a","a","b"], vec!["aa","b"]];
        assert_eq!(check, Solution::partition("aab".to_string()));

        assert_eq!(vec![vec!["a"]], Solution::partition("a".to_string()));
    }
}

