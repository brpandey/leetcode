
/*
23. Merge k Sorted Lists
Hard

Given an array of linked-lists, each linked list is sorted in ascending order.

Merge all the linked-lists into one sort linked-list and return it.



Example 1:

Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
1->4->5,
1->3->4,
2->6
]
merging them into one sorted list:
1->1->2->3->4->4->5->6

Example 2:

Input: lists = []
Output: []

Example 3:

Input: lists = [[]]
Output: [] 

 */

/*
lists
[list0, list1, list2, list3, list4, list5]

becomes: 
[list0(0,5), list1(1,4), list2(2,3)] -->
[list0(0,2), list1] -->
[list0(0,1)]
*/

#[path = "./p0021_merge_two_sorted_lists.rs"] pub mod p0021;
use p0021::Solution as previous;
use crate::util::ListNode;
use crate::util::ListNodeRef;

pub struct Solution {}

impl Solution {
    // pub type NodeLink<T> = Option<Box<ListNode<T>>>;
    pub fn merge_k_sorted(lists: Vec<Option<Box<ListNode>>>) -> ListNodeRef {
        let mut size = lists.len();

        if size == 0 { return None; }

        let mut l = lists;

        // finish only when one sorted list left
        while size > 1 {
            let (mut i, mut j) = (0, l.len() - 1);
            let mut merged = vec![];

            // Choose a pair of lists indexed by (i, j) and sort these and store in aux vec
            while i < j {
                // Since we can't move out of indexed content use take() so as not to leave holes
                merged.push(previous::sorted_merge(l[i].take(), l[j].take()));

                //If the previous markers were 0, k, its now at index 1, k-1, then 2, k-2 ..
                i += 1;
                j -= 1;
            }

            //                                         0  1  2  3  4
            // if say the list was odd or size 5 e.g. [a, b, c, d, e], our merged is [a+e, b+d], need to push c, i=2, j=2
            // if say the list was even or size 4 e.g.[a, b, c, d], our merged is [a+d, b+c], no need to push, i=2, j=1
            if i == j {
                merged.push(l[i].take())
            }

            l = merged; // set l to the intermediate list
            size = l.len(); // update
        }

        return l[0].take();
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::ListNode;

    #[test]
    fn test_0023(){
        let input = vec![
            ListNode::to_list(&[1,4,5]),
            ListNode::to_list(&[1,3,4]),
            ListNode::to_list(&[2,6])
        ];

        assert_eq!(ListNode::to_list(&[1, 1, 2, 3, 4, 4, 5, 6]), Solution::merge_k_sorted(input));
        assert_eq!(None, Solution::merge_k_sorted(vec![]));
        assert_eq!(None, Solution::merge_k_sorted(vec![None]));
    }
}
