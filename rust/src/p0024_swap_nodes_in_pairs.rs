#[path = "./p0021_merge_two_sorted_lists.rs"] pub mod lists;
use lists::Solution as util;


/*
24. Swap Nodes in Pairs
Medium

Share
Given a linked list, swap every two adjacent nodes and return its head.

You may not modify the values in the list's nodes, only nodes itself may be changed.


Example:

Given 1->2->3->4, you should return the list as 2->1->4->3.
 */

type NodeLink<T> = lists::NodeLink<T>;
type ListNode<T> = lists::ListNode<T>;

/*
pub type NodeLink<T> = Option<Box<ListNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
  pub data: T,
  pub next: Option<Box<ListNode<T>>>
}

*/

pub struct Solution {}

impl Solution {

    pub fn run(head: NodeLink<u32>) -> NodeLink<u32> {
        // This needs to be mut since we need a &mut pointing to it
        let mut dummy_head: NodeLink<u32> = ListNode::new(0);
        dummy_head.as_mut().unwrap().next = head;

        // current has to be a reference otherwise we get move errors
        // current is essentially x to begin with
        let mut current: &mut NodeLink<u32> = &mut dummy_head;

        // Do a check if current != None and current.next != None and current.next.next != None
        // x -> a -> b -> c -> d
        while current.is_some() // x != None
            && current.as_mut().unwrap().next.is_some() // a != None
            && current.as_mut().unwrap().next.as_mut().unwrap().next.is_some() { // b != None

                // params are current.next and current.next.next
                // take() allows us to take ownership of the value at struct field next, otherwise
                // we get move errors since we can't leave source uninitialized, have to replace it with None

                // for the first iteration first is a and second is b
                let mut first: NodeLink<u32> = current.as_mut().unwrap().next.take();
                let mut second: NodeLink<u32> = first.as_mut().unwrap().next.take();

                // current is x
                // since we took x.next it is None but is now b
                // this swap creates creates x -> b -> a -> c -> d
                current.as_mut().unwrap().next = Solution::swap(first, second);

                // current is now a or current.next.next
                current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
            }

        return dummy_head.unwrap().next;
    }

    // a -> b -> c, where first is a and second is b
    // changes to b -> a -> c
    pub fn swap(mut first: NodeLink<u32>, mut second: NodeLink<u32>) -> NodeLink<u32> {
        // a -> c
        first.as_mut().unwrap().next = second.as_mut().unwrap().next.take();
        // b -> a which points to c
        second.as_mut().unwrap().next = first;
        return second;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0024(){
        assert_eq!(util::to_list(&[2, 1, 4, 3]), Solution::run(util::to_list(&[1,2,3,4])));
    }
}
