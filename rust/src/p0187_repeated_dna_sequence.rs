// 187. Repeated DNA Sequences
//     Medium

//     All DNA is composed of a series of nucleotides abbreviated as A, C, G, and T, for example: "ACGAATTCCG". When studying DNA, it is sometimes useful to identify repeated sequences within the DNA.

//     Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.

//     Example:

// Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"

//     Output: ["AAAAACCCCC", "CCCCCAAAAA"]

use std::collections::{HashMap, HashSet};

pub const PRIME: i32 = 31;

pub struct Solution {}

impl Solution {

    // Essentially we create fingerprints of the sequences in the text and if have more than one
    // of the same fingerprint that is not a false positive we return it in our collection of
    // repeated dna sequences.  We use the Rabin-Karp String matching algorithm whose average case
    //is O(n + m)

    // https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm
    pub fn rabin_karp(text: &str, m: usize, prime: i32) -> Vec<String> {
        let t: &[u8] = text.as_bytes();
        let n = t.len();

        if n < m { return vec![]; }

        let mut results: Vec<String> = vec![];

        let map: HashMap<u8, i32> = [('A' as u8, 0), ('C' as u8, 1), ('G' as u8, 2), ('T' as u8, 3)]
            .iter().cloned().collect();

        // base 4 has only 4 values
        let base: i32 = map.len() as i32;
        let mut initial_hash = 0;
        let mut code;

        // E.g if base = 10 and m = 4, h => 1000
        // In essence this is the multiplier to remove the leading digit
        // Say the rolling hash up to this point was 3541 and the new digit is 2
        // If the leading digit is 3, 3*1000 = 3000, so you subtract by this amount so you can add the new digit in
        // 3541 - 3000 => 541, 541*10 + 2 => 5412, and the new rolling hash is still size m
        let h: i32 = base.pow(m as u32 -1) as i32;

        let mut unique = HashSet::new();

        // Preprocess uisng Horner's Rule (https://en.wikipedia.org/wiki/Horner%27s_method)
        // Get the hash/fingerprint for the first 10 characters/digits

        // In some sense our hash function is like a polynomial e.g.
        // 3x^4 + 1x^3 + 0x^2 + 3x + 2 = 31032 if x = 10 (as in base 10)
        // Since numbers like this can grow quickly,
        // we modulo with a prime number to bound the number somewhat and
        // minimize collisions as we are building up

        // Think of polynomial coefficients as the ascii value of a certain A, C, G, T character and the base as 4
        // So Tx^4 + Cx^3 + Ax^2 + Tx + G

        // Thus we generate a somewhat unique finger print of this values sequence (T, C, A, T, G)
        // THe unique thing about Horner's Rules is the polynomial e.g. 3x^4 + 1x^3 + 0x^2 + 3x + 2
        // Can be solved iteratively/recursively like (((3x + 1)x + 0)x + 3)x + 2
        // or ((((3*10 + 1)10 + 0)10 + 3)10 + 2) = 31032

        // Given a polynomial a(n)x^n + a(n-1)x^n-1 + ... + a(1)x + a(0)
        // Using Horner's Rule we can solve it like (similiar to a loop):
        // 1) result = a(n)
        // 2) result = result * x + a(n-1)
        // 3) result = result * x + a(n-2) .......  result = result * x + a(0)


        // In essence we are building up a number in O(m) time
        for i in 0..m {
            code = *map.get(&t[i]).unwrap();
            initial_hash = (initial_hash * base + code) % prime;
        }



        // Now we create a rolling hash which takes constant 0(1) time
        // And we track if we have seen a hash for our sliding window more than once
        // If this is so, then we add it to our results list!

        let mut rolling_hash = initial_hash;

        for i in 0..n-m+1 {
            let window = &t[i..i+m];

            // Add all the rolling_hashes and window sequences to a set
            // to make it easier to see if we have found one that occurs more than once
            let key = (rolling_hash, window);

            // Adding the hash is not enough as there could be a collision with another sequence
            // Hence we add the sequence as well

            // Every key gets inserted here

            if false == unique.insert(key) {
                results.push(String::from_utf8(window.to_vec()).unwrap()); // Convert &[u8] to Vec<u8> then to String
            }

            // Keep going
            if i + m < n {
                let leading = *map.get(&t[i]).unwrap();
                let trailing = *map.get(&t[i+m]).unwrap();

                // rh = d(rh − p[i + 1] * d^(m − 1)) + p[i + m]

                /*
                Given a text t = [3, 1, 4, 1, 5, 2] and m = 5, q = 13, d = 10
                ih = 31415
                rh = 10(31415 - 10^(5−1) * t[0]) + t[5]
                   = 10(31415 − 10^4 * 3) + 2
                   = 10(1415) + 2 = 14152
                 */

                rolling_hash = (base * (rolling_hash - (leading * h)) + trailing) % prime;
                rolling_hash = if rolling_hash < 0 { rolling_hash + prime } else { rolling_hash };
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_p0187() {
        assert_eq!(vec!["ATG"], Solution::rabin_karp("ATGATG", 3, PRIME));
        assert_eq!(vec!["AAAAACCCCC", "CCCCCAAAAA"], Solution::rabin_karp("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT", 10, PRIME));
    }
}
