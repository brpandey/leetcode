use std::collections::BTreeSet;

pub struct Solution {}

impl Solution {
    pub fn run(nums: &mut [i32]) -> Vec<[i32; 3]> {
        let mut uniques: BTreeSet<[i32; 3]> = BTreeSet::new(); //guarentees order which is helpful come assert_eq
        let mut results: Vec<[i32; 3]> = Vec::new();
        let (mut lo, mut hi) : (usize, usize);
        let (mut target, mut a, mut b, mut c): (i32, i32, i32, i32);
        let n = nums.len();

        // Make it easier to reason about the nums slice
        nums.sort();

        // Start from 0 just up until we can form our last triplet [n-3, n-2, n-1]
        for i in 0..n-2 {
            a = nums[i];

            if a > 0 {
                break;
            }

            // Essential we turn into a 2sum problem
            target = 0 - a;
            lo = i + 1;
            hi = n - 1;

            // Use the same approach from container with most water
            while lo < hi {
                b = nums[lo];
                c = nums[hi];

                if target == b + c {
                    // Filter out duplicate triplets
                    uniques.insert([a, b, c]);
                    lo += 1;
                } else if b + c < target{
                    lo += 1;
                } else {
                    hi -= 1;
                }
            }
        }

        // Move triplets now that we have uniquely identified them to results
        for u in uniques {
            results.push(u);
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0015(){
        let mut check: Vec<[i32; 3]> = Vec::new();
        check.push([-1, -1, 2]);
        check.push([-1, 0, 1]);
        assert_eq!(check, Solution::run(&mut [-1, 0, 1, 2, -1, -4]));
    }
}
