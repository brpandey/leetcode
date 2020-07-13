pub struct Solution {}

impl Solution {
    pub fn int_palindrome(mut num: i32) -> bool {
        // Handle special cases
        if num < 0 || (num % 10 == 0 && num != 0) {
            return false;
        }

        let mut reverse = 0;

        // If num is 123, we stop the while loop when num is 1 and reverse is 32 (odd)
        // If num is 181, we stop the while loop when num is 1 and reverse is 18 (odd)
        // If num is 1881, we stop the while loop when num is 18 and reverse is 18 (even)
        // If num is 1981, we stop the while loop when num is 1 and reverse is 189 (odd)
        // If num is 12321, we stop the while loop when num is 12 and reverse is 123 (odd)
        // If num is 12351, we stop the while loop when num is 12 and reverse is 153 (odd)
        while num > reverse {
            reverse = reverse * 10 + num % 10;
            num /= 10;
        }

        // Compare two halfs of palindrome accomodating even/odd cases
        // If num and reverse are even lengths we explicitly check for it
        // else we drop the middle number if odd, so given num is 12321, we have num is 12 and reverse is 12
        return (num == reverse) || (num == reverse / 10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0009() {
        assert_eq!(true, Solution::int_palindrome(121));
        assert_eq!(false, Solution::int_palindrome(-121));
        assert_eq!(false, Solution::int_palindrome(10));
        assert_eq!(true, Solution::int_palindrome(12321));
        assert_eq!(false, Solution::int_palindrome(12351));
        assert_eq!(true, Solution::int_palindrome(1881));
        assert_eq!(false, Solution::int_palindrome(1981));
    }
}
