#[path = "./p0021_merge_two_sorted_lists.rs"] pub mod two_sorted_lists;

/*
23. Merge k Sorted Lists
Hard

Given an array of linked-lists lists, each linked list is sorted in ascending order.

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

// Using code from p0021_merge_two_sorted_lists

type NodeLink<T> = two_sorted_lists::NodeLink<T>;

pub struct Solution {}

impl Solution {

    pub fn run(lists: &mut [NodeLink<u32>]) -> NodeLink<u32> {
        /*
        lists
        [list0, list1, list2, list3, list4, list5]
        
        becomes: 
        [list0(0,5), list1(1,4), list2(2,3)] -->
        [list0(0,2), list1] -->
        [list0(0,1)]
        */

        if lists.len() == 0 { return None; }
        let mut end = lists.len() - 1;

        while end != 0 {
            let mut i: usize = 0;
            let mut j: usize = end;

            // Basically we pluck a pair of lists indexed by (i, j) and sort these and store
            // the result back at i

            // We finish once there is only 1 list left in the array

            while i < j {
                lists[i] = two_sorted_lists::Solution::sorted_merge(lists[i].take(), lists[j].take());

                //If the old pair was 0, k, its now at index 1, k-1, then 2, k-2 ..
                i += 1;
                j -= 1;

                // if we've swept through the current iteration, set end to j
                // if we've finished all the sweep iterations, the last pair will be
                // i being 0 and j being 1, and hence the new pair will be i will be 1 and j will be 0

                if i >= j {
                    end = j;
                }
            }
        }

        return lists[0].take();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0023(){

        let mut y = [
            two_sorted_lists::Solution::to_list(&[1,4,5]),
            two_sorted_lists::Solution::to_list(&[1,3,4]),
            two_sorted_lists::Solution::to_list(&[2,6])
        ];
        assert_eq!(two_sorted_lists::Solution::to_list(&[1, 1, 2, 3, 4, 4, 5, 6]), Solution::run(&mut y));
        assert_eq!(None, Solution::run(&mut []));
        assert_eq!(None, Solution::run(&mut [None]));
    }
}
