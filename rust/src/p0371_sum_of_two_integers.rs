/*

371. Sum of Two Integers
Medium

Given two integers a and b, return the sum of the two integers without using the operators + and -.

Example 1:

Input: a = 1, b = 2
Output: 3

Example 2:

Input: a = 2, b = 3
Output: 5

Constraints:

-1000 <= a, b <= 1000

 */

/*
  Strategy

  XOR each of the bits - that indicates if anything is different since 0 ^ 1 = 1 and 1 ^ 0 = 1
  while the other combinations are 0.  Just need to track when there is a carry e.g. 1 + 1, do this by
  ANDing a and b and shifting it left over by one to be used for higher bit

  Do this sequence:
    a' = no_carry_sum ^| carry_sum
    b' = (no_carry_sum &| carry_sum) << 1

  E.g. 2 + 3

    0010       a    0010   a    0010 =>    a'   0001    a'   0001
  + 0011       b  ^ 0011   b  & 0011       b' ^ 0100    b' & 0100
  ------          ------      ------          ------       ------
     101       a'   0001   b'   0010       a''  0101    b''  0000 Stop! when b'' or carry_sum is 0

                                                            Answer is a'' or 0101 = 5
*/

pub struct Solution {}

impl Solution {
    pub fn get_sum(mut a: i32, mut b: i32) -> i32 {
        let mut no_carry_sum = a;
        let mut carry_sum = b;

        while carry_sum != 0 {
            a = no_carry_sum ^ carry_sum;
            b = (no_carry_sum & carry_sum) << 1;
            (no_carry_sum, carry_sum) = (a, b);
        }

        no_carry_sum
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0371() {
        assert_eq!(3, Solution::get_sum(1, 2));
        assert_eq!(5, Solution::get_sum(2, 3));
    }
}

