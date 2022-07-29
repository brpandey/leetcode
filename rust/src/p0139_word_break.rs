/*
139. Word Break
Medium

Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.

Note that the same word in the dictionary may be reused multiple times in the segmentation.



Example 1:

Input: s = "leetcode", wordDict = ["leet","code"]
Output: true
Explanation: Return true because "leetcode" can be segmented as "leet code".

Example 2:

Input: s = "applepenapple", wordDict = ["apple","pen"]
Output: true
Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
Note that you are allowed to reuse a dictionary word.

Example 3:

Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
Output: false



Constraints:

1 <= s.length <= 300
1 <= wordDict.length <= 1000
1 <= wordDict[i].length <= 20
s and wordDict[i] consist of only lowercase English letters.
All the strings of wordDict are unique.


*/


pub struct Solution {}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let slen = s.len(); // e.g. 7
        let mut dp = vec![false; slen+1]; // vec size 8, extra space for base case
        dp[slen] = true;  // this is base case, always set to true (e.g. s[7])

        // Say len(s) is 7

        // 0   1   2   3   4   5   6   7
        // [T] [ ] [T] [ ] [T] [ ] [ ] [T] // This should be the dp list
        //  h   i   h   i   p   e   n
        //  _               _

        // 0   1   2   3   4   5   6   7
        // [T] [ ] [ ] [T] [ ] [T] [ ] [T] // This should be the dp list
        //  p   e   n   h   i   h   i
        //  _           _

        // 0   1   2   3   4   5   6   7
        // [T] [ ] [T] [ ] [ ] [T] [ ] [T] // This should be the dp list
        //  h   i   p   e   n   h   i
        //  _           _

        // if dict words are ["hi", "pen"]

        for i in (0..slen).rev() { // start from string end
            for w in word_dict.iter() { // loop through dict
                let wlen = w.len();

                if i + wlen <= slen && // don't overshoot str len
                    &s[i..(i+wlen)] == w { // ensure s substring is word w
                        dp[i] = dp[i + wlen]; // chain together values, only works in the end if these are true
                    }

                if dp[i] {
                    break
                }
            }
        }

        return dp[0]
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0139() {
        let v1 = Solution::word_break(String::from("hipenhi"), vec!["hi".to_string(),"pen".to_string()]);
        assert_eq!(true, v1);

        let v1 = Solution::word_break(String::from("leetcode"), vec!["leet".to_string(),"code".to_string()]);
        assert_eq!(true, v1);

        let v1 = Solution::word_break(String::from("applepenapple"), vec!["apple".to_string(),"pen".to_string()]);
        assert_eq!(true, v1);

        let v1 = Solution::word_break(String::from("catsandog"),
                                      vec!["cats","dog","sand","and","cat"].into_iter()
                                      .map(|s| s.to_string())
                                      .collect());
        assert_eq!(false, v1);

    }
}

