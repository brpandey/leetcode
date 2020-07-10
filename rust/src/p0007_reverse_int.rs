




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
        let mut reversed = 0;
        for d in (I32ReverseDigits{value: val}) {
            reversed = reversed*10 + d;
        }

        reversed
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0007() {
        assert_eq!(321, Solution::reverse_int(123));
        assert_eq!(-321, Solution::reverse_int(-123));
        assert_eq!(21, Solution::reverse_int(120));
//        assert_eq!(1, Solution::reverse_int(2147483649));
//        assert_eq!(1, Solution::reverse_int(-2147483648));

    }
}
