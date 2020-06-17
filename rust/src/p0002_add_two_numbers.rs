use std::collections::LinkedList;

/*
https://leetcode.com/problems/add-two-numbers/

You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order and each of their nodes contain a single digit.
Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.
*/

pub struct Solution{}

impl Solution {

    pub fn add_two_numbers_list(list1: &LinkedList<u32>, list2: &LinkedList<u32>) -> LinkedList<u32> {
        let mut output: LinkedList<u32> = LinkedList::new();
        let mut iter1 = list1.iter().peekable();
        let mut iter2 = list2.iter().peekable();
        let mut carry: u32 = 0;
        let mut sum: u32;
        let mut temp_sum: u32 = 0;

        while iter1.peek().is_some() || iter2.peek().is_some() {
            if let Some(&v1) = iter1.next() {
                temp_sum += v1;
            }
            if let Some(&v2) = iter2.next() {
                temp_sum += v2;
            }
            sum = (temp_sum + carry) % 10;
            carry = (temp_sum + carry) / 10;

            temp_sum = 0;
            output.push_back(sum);
        }

        output
    }


    pub fn add_two_numbers(list1: &Vec<u32>, list2: &Vec<u32>) -> Vec<u32> {
        let l1: LinkedList<_> = list1.clone().into_iter().collect::<LinkedList<_>>();
        let l2: LinkedList<_> = list2.clone().into_iter().collect::<LinkedList<_>>();
        let output = Solution::add_two_numbers_list(&l1, &l2);
        output.into_iter().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_0002() {
        assert_eq!(vec![7, 0, 8], Solution::add_two_numbers(&vec![2, 4, 3], &vec![5, 6, 4]));
        assert_eq!(vec![7, 0, 8, 9], Solution::add_two_numbers(&vec![2, 4, 3], &vec![5, 6, 4, 9]));
        assert_eq!(vec![7, 0, 8, 7], Solution::add_two_numbers(&vec![2, 4, 3, 7], &vec![5, 6, 4]));
    }
}

