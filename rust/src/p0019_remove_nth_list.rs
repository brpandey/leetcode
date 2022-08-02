use std::cell::RefCell;
use std::rc::Rc;

type NodeRef<T> = Rc<RefCell<ListNode<T>>>;
type NodeLink<T> = Option<NodeRef<T>>;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode<T> {
    pub data: T,
    pub next: NodeLink<T>
}

impl<T> ListNode<T> {
    pub fn new(data: T) -> NodeRef<T> {
        Rc::new(
            RefCell::new(
                ListNode {
                    next: None,
                    data
                }
            )
        )
    }
}

pub struct Solution {}

impl Solution {
    pub fn run(head: &Option<NodeRef<u32>>, n: u32) -> Option<NodeRef<u32>> {

        let dummy: NodeRef<u32> = ListNode::new(0);
        dummy.borrow_mut().next = Some(Rc::clone(head.as_ref().unwrap()));
        let marker = Some(dummy);

        let mut first: Option<NodeRef<u32>> = Some(Rc::clone(head.as_ref().unwrap()));
        let mut second: Option<NodeRef<u32>>= Some(Rc::clone(head.as_ref().unwrap()));
        let mut temp: Option<NodeRef<u32>>;

        // Advance first so that first and second are n nodes apart

        // type NodeLink<T> = Option<Rc<RefCell<ListNode<T>>>>;
        for _ in 0..=n {
            first = Solution::next(&first);
        }

        // Move first to the list end keeping that gap of n nodes
        while first.is_some() {
            first = Solution::next(&first);
            second = Solution::next(&second);
        }

        temp = Solution::next(&second);
        temp = Solution::next(&temp);

        // Using interior mutability make immutable reference mutable (using runtime checks instead of compile time)
        // as_ref takes an Option<T> and turns it into a Option<&T>, so we can borrow the inside
        // unwrap takes the value inside the Option<&T> hence giving us a &T or &ListNode

        // temp already is Rc cloned
        second.as_ref().unwrap().borrow_mut().next = temp;

        Solution::next(&marker)
    }

    // type NodeRef<T> = Rc<RefCell<ListNode<T>>>;
    pub fn next(current: &Option<NodeRef<u32>>) -> Option<NodeRef<u32>>{
        if let Some(temp) = current {
            (temp.borrow().next).as_ref().map(|next| Rc::clone(next))
        } else {
            None
        }
    }

    pub fn to_list(a: &[u32]) -> Option<NodeRef<u32>> {
        let mut head: Option<NodeRef<u32>> = None;
        let mut n: NodeRef<u32>;

        // Reverse the array list so 4 then points to 5 etc..
        for v in a.iter().rev() {
            n = ListNode::new(*v);
            if head == None {
                n.borrow_mut().next = head;
            } else {
                n.borrow_mut().next = Some(Rc::clone(&head.unwrap()));
            }
            head = Some(n);
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0019(){
        assert_eq!(Solution::to_list(&[1, 2, 3, 4, 5]), Solution::run(&Solution::to_list(&[1, 2, 3, 4, 5]), 0));
        assert_eq!(Solution::to_list(&[1, 2, 3, 4]), Solution::run(&Solution::to_list(&[1, 2, 3, 4, 5]), 1));
        assert_eq!(Solution::to_list(&[1, 2, 3, 5]), Solution::run(&Solution::to_list(&[1, 2, 3, 4, 5]), 2));
        assert_eq!(Solution::to_list(&[1, 2, 4, 5]), Solution::run(&Solution::to_list(&[1, 2, 3, 4, 5]), 3));
        assert_eq!(Solution::to_list(&[1, 3, 4, 5]), Solution::run(&Solution::to_list(&[1, 2, 3, 4, 5]), 4));
    }
}

/*

array [1, 2, 3, 4, 5] is turned into this piece of ascii art-->

Some(
    RefCell {
        value: ListNode {
            data: 1,
            next: Some(
                RefCell {
                    value: ListNode {
                        data: 2,
                        next: Some(
                            RefCell {
                                value: ListNode {
                                    data: 3,
                                    next: Some(
                                        RefCell {
                                            value: ListNode {
                                                data: 4,
                                                next: Some(
                                                    RefCell {
                                                        value: ListNode {
                                                            data: 5,
                                                            next: None,
                                                        },
                                                    },
                                                ),
                                            },
                                        },
                                    ),
                                },
                            },
                        ),
                    },
                },
            ),
        },
    },
)

 */
