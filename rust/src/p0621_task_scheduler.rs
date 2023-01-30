use std::collections::{HashMap, BinaryHeap, VecDeque};
use std::iter::FromIterator;

/*
621. Task Scheduler
Medium

Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.

However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array), that is that there must be at least n units of time between any two same tasks.

Return the least number of units of times that the CPU will take to finish all the given tasks.

 

Example 1:

Input: tasks = ["A","A","A","B","B","B"], n = 2
Output: 8
Explanation: 
A -> B -> idle -> A -> B -> idle -> A -> B
There is at least 2 units of time between any two same tasks.

Example 2:

Input: tasks = ["A","A","A","B","B","B"], n = 0
Output: 6
Explanation: On this case any permutation of size 6 would work since n = 0.
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
And so on.

Example 3:

Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
Output: 16
Explanation: 
One possible solution is
A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A

 

Constraints:

    1 <= task.length <= 104
    tasks[i] is upper-case English letter.
    The integer n is in the range [0, 100].

*/

/*
Strategy:

Group tasks together and get counts
e.g. n = 2, ["A","A","A","B","B","C"] => "A" => 3, "B" => 2, "C" => 1

Process the counts that have the highest number first hence use a max heap,
process A first then B in our max heap

Store these in the max heap as ("A", 3), ("B", 2), ("C", 1) or just [3,2,1]
The letter is just a tag to identify the task and for printing a possible solution

Then pop from the max heap which would be ("A", 3)

Add this to the output or more succintly just update the time count from 0 to 1

Decrement the number of occurrences for this term so ("A", 3) goes to ("A", 2)
and then add to a waiting queue since we can't process another A for n time

Store a term ("A", 2, 1 (time) + 2 (n)) or ("A", 2, 3)
on the waiting queue such as queue [("A", 2, 3)]

So for time 1, check the wait queue to see if there's anything ready at time 1, no, nothing to push on to maxheap,

Pop the next max heap item which is ("B", 2), the time is incremented from 1 to 2
and stored in the wait queue with the term ("B", 1, 4)

Repeat and repeat
*/

pub struct Solution {}

impl Solution {
    pub fn least_interval(tasks: Vec<&str>, n: i32) -> i32 {
        let mut time: i32 = 0;

        // collect tasks into HashMap, incrementing count for each key
        let task_counts: HashMap<&str, u16> =
            tasks.into_iter().fold(HashMap::new(), |mut acc, t| {
                acc.entry(t).and_modify(|c| *c += 1).or_insert(1);
                acc
            });

        // flip the pair so that the binary heap tuple has the count first in the tuple
        // tuple ord comparison looks at the first element of the tuple
        let mut max_heap: BinaryHeap<(u16, &str)> =
            BinaryHeap::from_iter(task_counts.into_iter().map(|(ta, c)| (c, ta)));

        // (task name, count, wait-time)
        let mut waitq: VecDeque<(&str, u16, i32)> = VecDeque::new();

        // if we've just started waitq is empty and max_heap is not empty
        // if everything is waiting and no tasks are active, then max heap is empty and wait queue is not empty
        while !max_heap.is_empty() || !waitq.is_empty() {

            // if there's a matching task_tup on the wait queue
            // transfer it over to the binary heap as waiting is finished
            // and task can be actively considered
            if let Some((_, _, ti)) = waitq.front() {
                if *ti == time {
                    let (ta, c, _) = waitq.pop_front().unwrap();
                    max_heap.push((c, ta));
                }
            }

            time += 1;

            // grab next available task
            if let Some((mut c, ta)) = max_heap.pop() {
                // consume a task
//                print!("{} -> ", &ta);

                // now that we've "consumed" a task, decrement the number of its occurrences
                c -= 1;
                if c > 0 { // push only tasks that are remaining onto the wait queue
                    waitq.push_back((ta, c, time + n))
                }
            } else {
                // consume idle
//                print!("idle -> ");
            }
        }

//        println!("()");

        time
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0621() {
        // B -> A -> idle -> B -> A -> idle -> B -> A -> ()
        assert_eq!(8, Solution::least_interval(vec!["A","A","A","B","B","B"], 2));

        // B -> B -> B -> A -> A -> A -> ()
        assert_eq!(6, Solution::least_interval(vec!["A","A","A","B","B","B"], 0));

        // A -> G -> F -> A -> E -> D -> A -> C -> B -> A -> idle -> idle -> A -> idle -> idle -> A -> ()
        assert_eq!(16, Solution::least_interval(vec!["A","A","A","A","A","A","B","C","D","E","F","G"], 2));
    }
}
