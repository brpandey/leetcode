pub struct Solution {}

impl Solution {
    pub fn int_to_roman(val: usize) -> String {

        let mut roman: String = String::new();
        let digit1s: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        let digit10s: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        let digit100s: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        let digit1000s: [&str; 4] = ["", "M", "MM", "MMM"];

        //3218
        let d4 = digit1000s[val/1000 % 10]; // thousands digit
        let d3 = digit100s[val/100 % 10]; // hundreds digit
        let d2 = digit10s[val/10 % 10]; // tens digit
        let d1 = digit1s[val % 10]; // ones digit

        roman += d4;
        roman += d3;
        roman += d2;
        roman += d1;

        roman 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0012(){
        assert_eq!("III", Solution::int_to_roman(3));
        assert_eq!("IV", Solution::int_to_roman(4));
        assert_eq!("IX", Solution::int_to_roman(9 as usize));
        assert_eq!("LVIII", Solution::int_to_roman(58 as usize));
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994 as usize));
        assert_eq!("MMCCCXLIX", Solution::int_to_roman(2349 as usize));
    }
}
