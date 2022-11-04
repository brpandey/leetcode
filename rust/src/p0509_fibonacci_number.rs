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

use std::sync::{Arc, Mutex};


pub struct Solution {}

impl Solution {

    // Solution 1) Iterative
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

    // Solution 2a) Recursive with no memoization
    pub fn fib_recurse(n: u32) -> u32 {
        fn fib_helper1(n: u32) -> u32 {
            let value = match n {
                0 => 0,
                1 => 1,
                _ => fib_helper1(n - 1) + fib_helper1(n - 2),
            };

            value
        }
        // For base case, n=0, cache is &mut vec![None]
        fib_helper1(n)
    }

    // Solution 2b) Recursive with memoization
    pub fn fib_cache(n: u32) -> u32 {
        fn fib_helper2(n: u32, cache: &mut [Option<u32>]) -> u32 {
            // If not found, generate it through divide and conquer using cache memoization
            cache[n as usize].unwrap_or_else(|| {
                let value = match n {
                    0 => 0,
                    1 => 1,
                    _ => fib_helper2(n - 1, cache) + fib_helper2(n - 2, cache),
                };
                // Update cache
                cache[n as usize] = Some(value);
                value
            })
        }
        // For base case, n=0, cache is &mut vec![None]
        fib_helper2(n, &mut vec![None; n as usize + 1])
    }

    // Solution 3) Parallel + Recursive
    pub fn fib_parallel(n: u32) -> u32 {
        fn fib_cache_parallel(n: u32) -> u32 {
            fn fib_helper_parallel(n: u32, cache: Arc<Mutex<Vec<Option<u32>>>>) -> u32 {
                // If not found, generate it through divide and conquer using cache memoization
                let result;
                {
                    let unlocked = cache.lock().unwrap();
                    result = unlocked[n as usize];
                }

                match result {
                    Some(value) => value,
                    None => {
                        let value = match n {
                            0 => 0,
                            1 => 1,
                            _ => {
                                // Uses threadpool
                                // Create reference counted clones of our cache
                                let c1 = Arc::clone(&cache);
                                let c2 = Arc::clone(&cache);

                                let (a, b) = rayon::join(move || fib_helper_parallel(n - 1, c1), move || fib_helper_parallel(n - 2, c2));
                                a + b
                            }
                        };
                        // Update cache, attempt to get lock
                        {
                            let mut unlocked = cache.lock().unwrap();
                            unlocked[n as usize] = Some(value);
                        }
                        value
                    }
                }
            }
            // For base case, n=0, cache is &mut vec![None]
            let vec = vec![None; n as usize + 1];
            let cache: Arc<Mutex<Vec<Option<u32>>>> = Arc::new(Mutex::new(vec));
            // fib_helper_parallel(n, &mut vec![None; n as usize + 1])
            fib_helper_parallel(n, cache)
        }

        let thread_count = 4;
        let pool = rayon::ThreadPoolBuilder::new().num_threads(thread_count).build().unwrap();
        let n = pool.install(|| fib_cache_parallel(n));
        return n;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Instant};

    #[test]
    pub fn test_0509() {
        let prefix = "test p0509_fibonacci_number::tests::test_0509";
        let start = Instant::now();

        // Iterative
        assert_eq!(1, Solution::fib(2));
        assert_eq!(2, Solution::fib(3));
        assert_eq!(3, Solution::fib(4));
        assert_eq!(8, Solution::fib(6));
        assert_eq!(610, Solution::fib(15));
        assert_eq!(987, Solution::fib(16));
        // assert_eq!(832040, Solution::fib(30));
        // assert_eq!(102334155, Solution::fib(40));

        let duration = start.elapsed();
        println!("{} 1 Iter: {:?}", prefix, duration);
        let start = Instant::now();

        // Recursive with no memoization cache
        assert_eq!(1, Solution::fib_recurse(2));
        assert_eq!(2, Solution::fib_recurse(3));
        assert_eq!(3, Solution::fib_recurse(4));
        assert_eq!(8, Solution::fib_recurse(6));
        assert_eq!(610, Solution::fib_recurse(15));
        assert_eq!(987, Solution::fib_recurse(16));
        // assert_eq!(832040, Solution::fib_recurse(30));
        // assert_eq!(102334155, Solution::fib_recurse(40));

        let duration = start.elapsed();
        println!("{} 2a Recurse: {:?}", prefix, duration);
        let start = Instant::now();


        // Recursive with cache
        assert_eq!(1, Solution::fib_cache(2));
        assert_eq!(2, Solution::fib_cache(3));
        assert_eq!(3, Solution::fib_cache(4));
        assert_eq!(8, Solution::fib_cache(6));
        assert_eq!(610, Solution::fib_cache(15));
        assert_eq!(987, Solution::fib_cache(16));
        // assert_eq!(832040, Solution::fib_cache(30));
        // assert_eq!(102334155, Solution::fib_cache(40));

        let duration = start.elapsed();
        println!("{} 2b Memo: {:?}", prefix, duration);
        let start = Instant::now();

        // Parallel with memoization
        assert_eq!(1, Solution::fib_parallel(2));
        assert_eq!(2, Solution::fib_parallel(3));
        assert_eq!(3, Solution::fib_parallel(4));
        assert_eq!(8, Solution::fib_parallel(6));
        assert_eq!(610, Solution::fib_parallel(15));
        assert_eq!(987, Solution::fib_parallel(16));
        // assert_eq!(832040, Solution::fib_parallel(30));
        // assert_eq!(102334155, Solution::fib_parallel(40));

        let duration = start.elapsed();
        println!("{} 3 Parallel: {:?}", prefix, duration);
    }
}
