/*
Meeting Rooms II

Medium

Given an array of meeting time intervals consisting of start and end times[[s1,e1],[s2,e2],...](si< ei), find the minimum number of conference rooms required.

Example 1:

Input:
[[0, 30],[5, 10],[15, 20]]
Output:
2

Example 2:

Input:
[[7,10],[2,4]]
​
Output:
1
 */

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution {}

impl Solution {
    pub fn run(input: &mut Vec<[u8; 2]>) -> u8 {
        if input.is_empty() { return 0 };

        // Compare first element of tuple
        input.sort_by(|i1, i2| i1[0].cmp(&i2[0]));

        let mut heap = BinaryHeap::new();

        // Seed with the first interval's end time
        // Wrap with Reverse to make it a min heap

        let seed: u8 = input[0][1];

        // Essentially the first meeting to occur within the day gets an allocation on the heap
        // The heap is a reflection of how many separate meeting rooms we need
        heap.push(Reverse(seed));

        // Iterate through skipping the already processed seed
        for i in input.iter().skip(1) {
            let start = i[0];
            let end = i[1];
            if let Some(&Reverse(earliest_end)) = heap.peek() { // Look at the meeting which has the earliest end time slot thus far
                if earliest_end <= start { heap.pop(); } // If the earliest ending meeting finishes at or before the next meeting start, reuse the room!
            }

            heap.push(Reverse(end)); // Add current meeting time to heap
        }

        // Whatever is left on the heap after we have gone through the input vector are the items which overlapped
        // and hence could not reuse a room but needed their own room since their times were not mutually exclusive
        heap.len() as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn test_0253() {
        let mut input1: Vec<[u8; 2]> = [[0, 30],[5, 10],[15, 20]].iter().cloned().collect();
        let mut input2: Vec<[u8; 2]> = [[7,10],[2,4]].iter().cloned().collect();
        assert_eq!(2, Solution::run(&mut input1));
        assert_eq!(1, Solution::run(&mut input2));
    }
}
