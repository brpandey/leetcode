
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

use std::collections::LinkedList;

use crate::util::ListNode;


// Solution uses Box ListNode
pub struct Solution{}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut n1 = &l1;
        let mut n2 = &l2;

        let mut acc_carry: u32 = 0;
        let mut sum_digit: u32;
        let mut temp_sum: u32 = 0;

        let mut dummy_head = ListNode::new(-1);
        let mut tail = &mut dummy_head;

        while n1.is_some() || n2.is_some() {
            if let Some(ref v1) = n1 {
                temp_sum += v1.data as u32;
                n1 = &n1.as_ref().unwrap().next
            }

            if let Some(ref v2) = n2 {
                temp_sum += v2.data as u32;
                n2 = &n2.as_ref().unwrap().next
            }

            sum_digit = (temp_sum + acc_carry) % 10; // store the sum digit in newly created node
            acc_carry = (temp_sum + acc_carry) / 10; // track the carry to be used on next loop iteration

            tail.as_mut().unwrap().next = ListNode::new(sum_digit as i32);  // create the list chain
            tail = &mut tail.as_mut().unwrap().next; // update the tail ptr to point to last element in chain

            temp_sum = 0; // important to reset
        }

        dummy_head.as_mut().unwrap().next.take() // return head of new list
    }
}

// Solution uses LinkedList type from std collections
pub struct Solution1{}

impl Solution1 {

    pub fn add_two_numbers_list(list1: &LinkedList<&u32>, list2: &LinkedList<&u32>) -> LinkedList<u32> {
        let mut output: LinkedList<u32> = LinkedList::new();
        let mut iter1 = list1.iter(); //.peekable();
        let mut iter2 = list2.iter(); //.peekable();
        let mut n1 = iter1.next();
        let mut n2 = iter2.next();

        let mut carry: u32 = 0;
        let mut sum: u32;
        let mut temp_sum: u32 = 0;

        while n1.is_some() || n2.is_some() {
            if let Some(&v1) = n1 {
                temp_sum += v1;
                n1 = iter1.next();
            }

            if let Some(&v2) = n2 {
                temp_sum += v2;
                n2 = iter2.next();
            }

            sum = (temp_sum + carry) % 10;
            carry = (temp_sum + carry) / 10;

            temp_sum = 0;
            output.push_back(sum);
        }

        output
    }


    pub fn add_two_numbers(list1: &Vec<u32>, list2: &Vec<u32>) -> Vec<u32> {
        let l1: LinkedList<_> = list1.iter().collect::<LinkedList<_>>();
        let l2: LinkedList<_> = list2.iter().collect::<LinkedList<_>>();
        let output = Solution1::add_two_numbers_list(&l1, &l2);
        output.into_iter().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0002() {
        // Solution1
        assert_eq!(vec![7, 0, 8], Solution1::add_two_numbers(&vec![2, 4, 3], &vec![5, 6, 4]));
        assert_eq!(vec![7, 0, 8, 9], Solution1::add_two_numbers(&vec![2, 4, 3], &vec![5, 6, 4, 9]));
        assert_eq!(vec![7, 0, 8, 7], Solution1::add_two_numbers(&vec![2, 4, 3, 7], &vec![5, 6, 4]));

        // Solution
        assert_eq!(ListNode::to_list(&[7, 0, 8]),
                   Solution::add_two_numbers(ListNode::to_list(&[2, 4, 3]), ListNode::to_list(&[5, 6, 4])));
        assert_eq!(ListNode::to_list(&[7, 0, 8, 9]),
                   Solution::add_two_numbers(ListNode::to_list(&[2, 4, 3]), ListNode::to_list(&[5, 6, 4, 9])));
        assert_eq!(ListNode::to_list(&[7, 0, 8, 7]),
                   Solution::add_two_numbers(ListNode::to_list(&[2, 4, 3, 7]), ListNode::to_list(&[5, 6, 4])));
    }
}

