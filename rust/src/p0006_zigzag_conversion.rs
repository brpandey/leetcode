/**
 * 6 ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 *
 *
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 *
 *
 * And then read line by line: "PAHNAPLSIIGYIR"
 *
 * Write the code that will take a string and make this conversion given a number of rows:
 *
 *
 * string convert(string s, int numRows);
 *
 * Example 1:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 *
 *
 * Example 2:
 *
 *
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 *
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 *
 * The above pattern is converted into the
 * steps index pattern
 *
 * 0     0     0
 * 1   1 1   1 1
 * 2 2   2 2
 * 3     3
 *
 * Zip it with the grapheme such that a pair row, ch
 * is accessible
 *
 * Each row as indentified by the unique steps index number
 * can be stored in a map using the row as the key
 *
 * The output can then be derived by concatenating each row's values
 * to each other (each row can be viewed as a vector of values)
 */
use unicode_segmentation::UnicodeSegmentation;
use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn zigzag(input: &str, n: usize) -> String {

        if n == input.len() { return input.to_string() }

        let step_down = (1..n-1).rev();
        let steps = (0..n).chain(step_down).cycle();
        let zipped = steps.zip(input.graphemes(true).clone());

        let map = zipped.fold(BTreeMap::new(), |mut map, (row, ch)| {
            let val = map.entry(row).or_insert(vec![]);
            val.push(ch);
            map
        });

        map.iter().fold(String::new(), |mut s, (_, v)| {
            s.push_str(&v.join(""));
            s
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0006() {
        assert_eq!("PAYPALISHIRING", Solution::zigzag("PAYPALISHIRING", 1));
        assert_eq!("PYAIHRNAPLSIIG", Solution::zigzag("PAYPALISHIRING", 2));
        assert_eq!("PAHNAPLSIIGYIR", Solution::zigzag("PAYPALISHIRING", 3));
        assert_eq!("PINALSIGYAHRPI", Solution::zigzag("PAYPALISHIRING", 4));
    }
}
