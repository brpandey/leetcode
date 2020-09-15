//     509. Fibonacci Number

//     The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

// F(0) = 0,   F(1) = 1
//     F(N) = F(N - 1) + F(N - 2), for N > 1.

//     Given N, calculate F(N).

//     Example 1:

// Input: 2
//     Output: 1
//     Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.

//     Example 2:

// Input: 3
//     Output: 2
//     Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.

//     Example 3:

// Input: 4
//     Output: 3
//     Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

    

//     Note: 0 ≤ N ≤ 30.

pub struct Solution {}

impl Solution {
    pub fn fib(mut n: u32) -> u32 {
        let mut stack = Vec::with_capacity(n as usize + 1);
        let (mut a, mut b) : (u32, u32);
        stack.push(0);
        stack.push(1);

        //  0, 1, 2, 3, 4, 5, 6,  7,  8,  9, 10, 11,  12,  13,  14,  15,  16, ...
        //  0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, ... 
        while n > 1 {
            b = stack.pop().unwrap(); // 1 base case
            a = stack.pop().unwrap(); // 0 base case
            stack.push(b);
            stack.push(b + a);
            n -= 1;
        }
        stack.pop().unwrap()
    }

    pub fn fib_cache(n: u32) -> u32 {
        fn fib_helper(n: u32, cache: &mut [Option<u32>]) -> u32 {
            // If not found, generate it through divide and conquer using cache memoization
            cache[n as usize].unwrap_or_else(|| {
                let value = match n {
                    0 => 0,
                    1 => 1,
                    _ => fib_helper(n - 1, cache) + fib_helper(n - 2, cache),
                };
                // Update cache
                cache[n as usize] = Some(value);
                value
            })
        }
        // For base case, n=0, cache is &mut vec![None]
        fib_helper(n, &mut vec![None; n as usize + 1])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0509() {
        // Iterative
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
        assert_eq!(8, Solution::fib(6));
        assert_eq!(610, Solution::fib(15));
        assert_eq!(987, Solution::fib(16));
        assert_eq!(832040, Solution::fib(30));

        // Recursive with cache
        assert_eq!(1, Solution::fib_cache(2));
        assert_eq!(2, Solution::fib_cache(3));
        assert_eq!(3, Solution::fib_cache(4));
        assert_eq!(8, Solution::fib_cache(6));
        assert_eq!(610, Solution::fib_cache(15));
        assert_eq!(987, Solution::fib_cache(16));
        assert_eq!(832040, Solution::fib_cache(30));
    }
}
