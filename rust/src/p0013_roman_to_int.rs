use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(roman: &str) -> u32 {
        let table: HashMap<char, u32> = [('I', 1), ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)].iter().cloned().collect();
        let mut current: &u32 = &0;

        let (value, _) = roman.chars().fold((0, &0), |(mut acc, last), ch| {
            current = table.get(&ch).unwrap();

            // Case: Current Roman numeral is less than next char (IX or IV or XL etc..)

            // Undo having added the 'I' now seeing it in its "IX" context
            // E.g. [I, X], we are at X, given acc is 1 due to I,
            // substract 2*1=2 and then add current which is 10 => 1 + 10 - 2 = 9
            if last < current {
                acc += current - 2*last;
            } else {
                acc += current;
            }
            (acc, current)
        });

        value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0013(){
        assert_eq!(3, Solution::roman_to_int("III"));
        assert_eq!(4, Solution::roman_to_int("IV"));
        assert_eq!(9, Solution::roman_to_int("IX"));
        assert_eq!(58, Solution::roman_to_int("LVIII"));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV"));
        assert_eq!(2349, Solution::roman_to_int("MMCCCXLIX"));
    }
}
