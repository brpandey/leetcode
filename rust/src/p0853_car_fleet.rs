/*
 * 853. Car Fleet
Medium

There are n cars going to the same destination along a one-lane road. The destination is target miles away.

You are given two integer array position and speed, both of length n, where position[i] is the position of the ith car and speed[i] is the speed of the ith car (in miles per hour).

A car can never pass another car ahead of it, but it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).

A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.

If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.

Return the number of car fleets that will arrive at the destination.

 

Example 1:

Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
Output: 3
Explanation:
The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting each other at 12.
The car starting at 0 does not catch up to any other car, so it is a fleet by itself.
The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
Note that no other cars meet these fleets before the destination, so the answer is 3.

Example 2:

Input: target = 10, position = [3], speed = [3]
Output: 1
Explanation: There is only one car, hence there is only one fleet.

Example 3:

Input: target = 100, position = [0,2,4], speed = [4,2,1]
Output: 1
Explanation:
The cars starting at 0 (speed 4) and 2 (speed 2) become a fleet, meeting each other at 4. The fleet moves at speed 2.
Then, the fleet (speed 2) and the car starting at 4 (speed 1) become one fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.

// position,speed sorted => [(4,1),(2,2),(0,4)]
// 100-4/1, 100-2/2,100-0/4 => 96,49,25

Constraints:

    n == position.length == speed.length
    1 <= n <= 105
    0 < target <= 106
    0 <= position[i] < target
    All the values of position are unique.
    0 < speed[i] <= 106


 */

/*
 * Strategy
 *
 * Given target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
 * 
 * Zip position and speed together so that we can sort on position
 *
 * [(10,2),(8,4),(0,1),(5,1),(3,3)] then sort by tuple.0 =>
 * [(10,2),(8,4),(5,1),(3,3),(0,1)]
 *
 * Calculate the number of hours it takes for each car to reach target
 * given the equation (position-target)/speed,
 *
 * 12-10/2, 12-8/4, 12-5/1, 12-3/3, 12-0/1 => 1,1,7,3,12
 *
 * Looking at the numbers 1,1,7,3,12,  
 * the first two cars reach the target at the same time so they're 1 fleet,
 * the car going at speed 3 won't be able to overtake the car
 * going at speed 7 since it is in front of it, so they become one fleet, 
 *
 * So if number after item in list is greater than add 1 to fleet count, init fleet count to 1
 */


pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // zip two vectors and sort by position, 
        // with the car farther ahead first (descending order)
        let mut zipped: Vec<(i32,i32)> = position.into_iter().zip(speed.into_iter()).collect();
        zipped.sort_by(|(p1,_s1), (p2, _s2)| p2.cmp(p1));
    
        let mut hours = Vec::with_capacity(zipped.len());

        // calculate # hours to target destination for each car
        for (p,s) in zipped {
            hours.push((target-p)/s)    
        }

        let (mut fleet, mut prev) = (0, 0);

        for h in hours {
            // if current hour h is longer than previous hour, 
            // the previous car speed is faster and 
            // the current h car can never catch up to the previous car so
            // increase fleet count
            if h > prev {
                fleet += 1;
            }

            prev = h;
        }

        fleet
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_p0853() {
        assert_eq!(3, Solution::car_fleet(12, vec![10,8,0,5,3], vec![2,4,1,1,3]));
        assert_eq!(1, Solution::car_fleet(100, vec![3], vec![3]));
        assert_eq!(1, Solution::car_fleet(100, vec![0,2,4], vec![4,2,1]));
    }
}

