/*
 * Encode and Decode Strings

Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.

Please implement encode and decode

Example1

Input: ["lint","code","love","you"]
Output: ["lint","code","love","you"]

Explanation:

One possible encode method is: "lint:;code:;love:;you"

Example2

Input: ["we", "say", ":", "yes"]
Output: ["we", "say", ":", "yes"]

Explanation:

One possible encode method is: "we:;say:;:::;yes"
 */

pub struct Solution {}

impl Solution {
    pub fn encode(list: &Vec<String>) -> String {
        let mut result: String = String::new();

        for l in list {
            let s = format!("{}#{}", l.len(), l);
            result += &s;
        }

        result
    }

    // e.g. string => "4#lint4#code4#love3#you"
    pub fn decode(s: String) -> Vec<String> {
        let sb: Vec<u8> = s.as_bytes().iter().cloned().collect();
        let (sblen, mut i) = (sb.len(), 0);
        let mut result = vec![];

        while i < sblen {
            let mut j = i;

            while j < sblen - 1 && *sb.get(j).unwrap() != '#' as u8 {
                j += 1;
            }

            let length: usize = (sb.get(i).unwrap() - b'0') as usize; // assume length is a number that fits in a u8 ~ < 256

            /*
            For longer individual text lengths > 9
            let length: usize = sb.get(i..j).unwrap().iter()
                .map(|&byte| byte - b'0')
                .fold(0, |mut acc, n| {acc = acc * 10 + n as usize; acc});
            */

            let s = std::str::from_utf8(&sb[j+1..j+length+1]).unwrap();
            result.push(s.to_string());

            i = j+length+1;
        }

        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0271() {
        let list = vec!["lint", "code", "love", "you"].into_iter().map(String::from).collect();
        assert_eq!(list, Solution::decode(Solution::encode(&list)));

        let list = vec!["we", "say", ":", "yes"].into_iter().map(String::from).collect();
        assert_eq!(list, Solution::decode(Solution::encode(&list)));
    }
}
