use std::collections::HashMap;
use std::collections::VecDeque;




// DFS Solution
use std::iter::FromIterator;

pub struct Solution0 {}

impl Solution0 {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];
        let tuple_list: [(u8, &str); 10] = [(0, ""), (1, ""), (2, "abc"), (3, "def"),
                                     (4, "ghi"), (5, "jkl"), (6, "mno"), (7, "pqrs"), (8, "tuv"), (9, "wxyz")];

        let mapping: HashMap<u8, &str> = HashMap::from_iter(tuple_list);
        let mut current = vec![];

        if digits.len() > 0 {
            Self::dfs(0, &mut current, &mapping, digits.as_bytes(), &mut result);
        }

        result
    }

    pub fn dfs(i: usize, cur: &mut Vec<u8>, mapping: &HashMap<u8, &str>, digits: &[u8], res: &mut Vec<String>) {
        // Base case
        // 'ad' or 'ae'    '23'
        if cur.len() == digits.len() {
            res.push(String::from_utf8(cur.clone()).unwrap());
            return
        }

        // e.g 2 => "abc"
        let letters = mapping.get(&(digits[i] - b'0')).unwrap();

        // e.g. "abc"
        for b in letters.bytes() {
            cur.push(b);
            Self::dfs(i+1, cur, &mapping, digits, res);
            cur.pop();
        }
    }
}

/*
 * ""abc
 *
 * pop a, code is def, hence => ad, ae, af
 *
 * Note A
 * Create the auxiliary queue before we iterate through main queue
 * This allows us to have remove from the main queue (1 mutable reference)
 * And insert onto an auxiliary queue (1 mutable reference)
 * (Preventing us from compiler errors about potential data race issues if we tried to simultaneously
 * Pop off from main and push onto main - or 2 mutable references aka not allowed)
 */

// BFS Solution 

pub struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: &str) -> Vec<String> {
        let at: [(u32, &str); 10] = [(0, ""), (1, ""), (2, "abc"), (3, "def"),
                                     (4, "ghi"), (5, "jkl"), (6, "mno"), (7, "pqrs"), (8, "tuv"), (9, "wxyz")];
        let mapping: HashMap<u32, &str> = at.iter().cloned().collect();
        let (mut queue, mut aux) : (VecDeque<String>, VecDeque<String>);
        let seed: String = String::from("");
        let n = digits.len();
        let mut code: &str;
        let mut digit: u32;
        let mut prefix: String;

        if n == 0 {
            return vec![];
        }

        queue = vec![seed].into();

        // Step through each of the input digits e.g. "23"
        for (i, d) in digits.chars().enumerate() {
            digit = d.to_digit(10).unwrap() as u32;
            code = mapping.get(&digit).unwrap();

            // See Note A
            aux = VecDeque::new();

            // first "", then "a", "b", "c", then "ad", "ae", "af", "bd", "be", "bf", ....
            while let Some(entry) = queue.pop_front() {
                // Given input "23" this would be the characters in "abc" or "def"
                for c in code.chars() {
                    prefix = entry.clone();

                    if i == prefix.len() {
                        prefix.push(c); // "a" now becomes "ad" or "ae" or "af"
                        aux.push_back(prefix);
                    }
                }
            }
            queue.append(&mut aux);
        }
        queue.iter().cloned().collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0017(){
        let check: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].iter().map(|x| x.to_string()).collect();
        assert_eq!(check.clone(), Solution::letter_combinations("23"));

        assert_eq!(check, Solution0::letter_combinations("23".to_string()));
    }
}
