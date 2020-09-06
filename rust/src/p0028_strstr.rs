/*
28. Implement strStr()
Easy

Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "hello", needle = "ll"
Output: 2

Example 2:

Input: haystack = "aaaaa", needle = "bba"
Output: -1

Clarification:

What should we return when needle is an empty string? This is a great question to ask during an interview.

For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().


 */

use unicode_segmentation::UnicodeSegmentation;

pub struct Solution {
}

impl Solution {
    pub fn str_str(haystack: &str, needle: &str) -> i32 {

        if needle.is_empty() {
            return 0;
        }

        let substr = haystack.graphemes(true).collect::<Vec<&str>>();
        let token = needle.graphemes(true).collect::<Vec<&str>>();
        let m: usize = substr.len();
        let n: usize = token.len();

        //  01234
        //5 apple
        //3 ple
        //must include index 2
        for i in 0..=(m-n) {

            // Use vector slices
            // https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with
            if substr[i..].starts_with(&token[..]) {
                return i as i32;
            }

            /* Another approach with more explicit steps
            for j in 0..n {
                // we've kept looping until we've seen all of token and have had it match
                if j + 1 == n {
                    return i as i32;
                }

                if i + j == m {
                    return -1;
                }

                if substr[(i+j)..(i+j+1)] != token[j..(j+1)] {
                    break;
                } 
            }
            */
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0028(){
        assert_eq!(2, Solution::str_str("hello", "ll"));
        assert_eq!(-1, Solution::str_str("aaaaa", "bba"));
        // handles unicode, as_bytes would have given 11 for the 11th byte,
        // 7 indicates the 7th grapheme
        assert_eq!(7, Solution::str_str("Zażółć gęślą jaźń", "gęślą"));
    }
}

