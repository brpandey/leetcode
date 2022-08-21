/*
 * 295. Find Median from Data Stream
Hard

The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value and the median is the mean of the two middle values.

    For example, for arr = [2,3,4], the median is 3.
    For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.

Implement the MedianFinder class:

    MedianFinder() initializes the MedianFinder object.
    void addNum(int num) adds the integer num from the data stream to the data structure.
    double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.

Example 1:

Input
["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
[[], [1], [2], [], [3], []]
Output
[null, null, null, 1.5, null, 2.0]

Explanation
MedianFinder medianFinder = new MedianFinder();
medianFinder.addNum(1);    // arr = [1]
medianFinder.addNum(2);    // arr = [1, 2]
medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
medianFinder.addNum(3);    // arr[1, 2, 3]
medianFinder.findMedian(); // return 2.0 

Constraints:

    -105 <= num <= 105
    There will be at least one element in the data structure before calling findMedian.
    At most 5 * 104 calls will be made to addNum and findMedian.

Follow up:

    If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
    If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?


 *
 */

/*
 * Keep two binaryheaps, one max heap for everything loosely to the left 
 * of the median and a min heap for everything loosely to the right of the median
 *
 * Idea is to balance the two binary heaps upon each add num call
 *
 * add operation is O(log n)
 * find median operation is O(1)
 */


use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MedianFinder {
    pub left_max: BinaryHeap<i32>,
    pub right_min: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            left_max: BinaryHeap::new(),
            right_min: BinaryHeap::new(),
        }
    }

    // Perform integrity checks on each operation
    pub fn add_num(&mut self, num: i32) {

        // Always push onto left
        self.left_max.push(num);

        /* Then check integrity constraints */

        // Only need to check for one element as function is called on each insert
        // Ensure left_max values are all <= right_min's values
        let left = self.left_max.peek();
        let mut right = None;

        if let Some(Reverse(r)) = self.right_min.peek() {
            right = Some(r);
        }

        // push bigger element from left onto right
        if left.is_some() && right.is_some() && left.unwrap() > right.unwrap() {
            if let Some(switch) = self.left_max.pop() {
                self.right_min.push(Reverse(switch));
            }
        }
        
        // ensure left_max size is not more than 1 than right_min's size
        if self.left_max.len() > self.right_min.len() + 1 {
            if let Some(switch) = self.left_max.pop() {
                self.right_min.push(Reverse(switch));
            }
        }

        // ensure right_max size is not more than 1 than left_max's size
        if self.right_min.len() > self.left_max.len() + 1 {
            if let Some(Reverse(switch)) = self.right_min.pop() {
                self.left_max.push(switch);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        let error = 0.0;

        // e.g. 3 > 2 or 4 > 3 notice that these both sum to odd lengths (lens have to be at most
        // + 1 from each other)
        if self.left_max.len() > self.right_min.len() {
            return *self.left_max.peek().unwrap() as f64
        }

        if self.right_min.len() > self.left_max.len() {
            if let Some(Reverse(v)) = self.right_min.peek() {
                return *v as f64
            }
        }

        if self.left_max.peek().is_some() && self.right_min.peek().is_some() {
            if let Some(Reverse(right)) = self.right_min.peek() {
                let left = self.left_max.peek().unwrap();
                return (*left as f64 + *right as f64)/2.0 
            }
        }

        error
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0295() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(1.5, mf.find_median());
        mf.add_num(3);
        assert_eq!(2.0, mf.find_median());
    }
}

