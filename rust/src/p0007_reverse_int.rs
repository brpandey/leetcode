/*
7. Reverse Integer
Medium

Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Example 1:

Input: x = 123
Output: 321

Example 2:

Input: x = -123
Output: -321

Example 3:

Input: x = 120
Output: 21

Constraints:

-231 <= x <= 231 - 1
*/

struct I32ReverseDigits {
    value: i32
}

impl Iterator for I32ReverseDigits {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.value == 0 {
            return None;
        }

        let result = Some(self.value % 10);
        self.value /= 10;
        result
    }
}

pub struct Solution {}

impl Solution {
    pub fn reverse_int(val: i32) -> i32 {
        let mut reversed: i32 = 0;
        let digits = I32ReverseDigits{value: val};
        for d in digits {
            reversed = match reversed.checked_mul(10) {
                Some(m) => m + d,
                None => 0,
            }
        }

        reversed
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 2147483647 is 2^31 -1 or i32::max_value
    fn test_0007() {
        assert_eq!(321, Solution::reverse_int(123));
        assert_eq!(-321, Solution::reverse_int(-123));
        assert_eq!(21, Solution::reverse_int(120));
        assert_eq!(0, Solution::reverse_int(i32::max_value()));
        assert_eq!(0, Solution::reverse_int(i32::min_value()));
        assert_eq!(219078635, Solution::reverse_int(i32::pow(2,29)));
        assert_eq!(1047483641, Solution::reverse_int(1463847401));
    }
}
