pub struct Solution {}

use std::collections::HashMap;

//                     0      1         2                    3
pub type Env<'a> = (usize, usize, &'a Vec<Vec<u16>>, &'a mut HashMap<(i16, i16), i16>);

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<u16>>) -> i16 {
        let mut dp = HashMap::new();

        let max_rows = matrix.len();
        let max_cols = matrix[0].len();

        // env allows us to close on the scope of these variables declared here
        // without having to pass them all in explicitly
        let mut env: Env = (max_rows, max_cols, &matrix, &mut dp);

        for r in 0..max_rows {
            for c in 0..max_cols {
                Self::dfs(r as i16, c as i16, 0, &mut env);
            }
        }

        *dp.values().max().unwrap()
    }

    pub fn dfs(r: i16, c: i16, prev: u16, env: &mut Env) -> i16 {
        use std::cmp::max;

        // sanity check that current r,c position is in bounds
        if r == env.0 as i16 || c == env.1 as i16 ||
            r < 0 || c < 0 ||
            env.2[r as usize][c as usize] <= prev { // important: must be greater than prev to be viable
                return 0
            }

        let (row, col) = (r as usize, c as usize);

        // if already cached don't re-compute, use cached value
        if env.3.contains_key(&(r,c)) {
            return *env.3.get(&(r,c)).unwrap()
        }

        let mut lip = 1;

        // explore neighboring cells of 4 directions, left, right, down, up
        lip = max(lip, 1 + Self::dfs(r, c-1, env.2[row][col], env));
        lip = max(lip, 1 + Self::dfs(r, c+1, env.2[row][col], env));
        lip = max(lip, 1 + Self::dfs(r-1, c, env.2[row][col], env));
        lip = max(lip, 1 + Self::dfs(r+1, c, env.2[row][col], env));

        env.3.insert((r,c), lip);

        lip
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0329() {
        let matrix = vec![vec![9,9,4],vec![6,6,8],vec![2,1,1]];
        assert_eq!(4, Solution::longest_increasing_path(matrix));

        let matrix = vec![vec![3,4,5],vec![3,2,6],vec![2,2,1]];
        assert_eq!(4, Solution::longest_increasing_path(matrix));

        let matrix = vec![vec![1]];
        assert_eq!(1, Solution::longest_increasing_path(matrix));

    }
}

