/*
[0118] Pascal's Triangle

Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it.

Input: 5

Output:
  [
1       [1],
2      [1,1],
3     [1,2,1],
4    [1,3,3,1],
5   [1,4,6,4,1]
  ]

*/

pub struct Solution {}

impl Solution {
    pub fn run(n: i32) -> Vec<Vec<i32>> {
        // Base case is the first iteration, return early
        let base = vec![vec![1]];

        if n == 1 {
            return base;
        }

        // Inductive case
        let mut rows = base;

        // OUTER LOOP => CREATES ROWS

        // If we are asking for 2 rows we already have the first 1
        // so we only need 1 iteration, 3 rows => 2 iterations (row 2 and row 3), etc..
        // NOTE: Since the first row is already created i starts at 1
        for i in 1..(n as usize) {

            // Create vector for the current row or "crow"
            let mut crow = Vec::new();
            let mut sum_child;

            // Row 1 has 1 element, Row 2 has 2 elements
            // If we are creating the third row, e.g. n = 3, we need three elements, however
            // two are the edge elements 1, so we only need one element computed,
            // For n = 4, we need 2 elements computed

            // Every row starts with a 1
            crow.push(1);

            // INNER LOOP => CREATES ELEMENTS IN CURRENT ROW
            // For row 2, i => 1, so the loop isn't executed which is what we want
            // For row 3, i => 2, so the loop runs once
            // For row 4, i => 3, so the loop runs twice
            // NOTE: Since the first row entry is already created j starts at 1
            for j in 1..i {
                // If this is say row 3 (at index 2), grab row 2 which is at index 1 or just i-1 and j-1 (for the left ancestor)
                // along with row 2 and j (for the right ancestor) which notice has the same j index
                // 0       [1],       # row 1
                // 1      [1,1],      # row 2
                // 2     [1,2,1],     # row 3
                // 3    [1,3,3,1],    # row 4

                sum_child = rows[i-1][j-1] + rows[i-1][j];
                crow.push(sum_child);
            }

            // Every row ends with a 1, push current row onto collection
            crow.push(1);
            rows.push(crow);
        }

        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p0118(){
        assert_eq!(vec![vec![1], vec![1,1]], Solution::run(2));
        assert_eq!(vec![vec![1], vec![1,1], vec![1,2,1], vec![1,3,3,1], vec![1,4,6,4,1]], Solution::run(5));
    }
}
