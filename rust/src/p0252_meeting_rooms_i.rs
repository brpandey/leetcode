/*
Given an array of meeting time intervals consisting of start and end times [[s1,e1],[s2,e2],...] (si < ei), determine if a person could attend all meetings.

For example,

Given [[0, 30],[5, 10],[15, 20]],
return false.

Given [[7,10],[2,4]]
​return true

 */

pub struct Solution {}

impl Solution {
    pub fn run(input: &mut Vec<[u8; 2]>) -> bool {

        let n = input.len();

        // Sort vector time slots by start time so we can efficiently compare them
        input.sort_by(|i1, i2| i1[0].cmp(&i2[0]));

        // loop through vector grabbing the next two items at a time
        for i in 0..n-1 {
            let first = input[i];
            let second = input[i+1];

            // test for any overlaps, if so, we can't make all meetings if there is an overlap in meeting time
            // end1 > start2
            if first[1] > second[0] { return false };
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0252() {
        let mut input1: Vec<[u8; 2]> = [[0, 30],[5, 10],[15, 20]].iter().cloned().collect();
        let mut input2: Vec<[u8; 2]> = [[7,10],[2,4]].iter().cloned().collect();
        assert_eq!(false, Solution::run(&mut input1));
        assert_eq!(true, Solution::run(&mut input2));
    }
}
