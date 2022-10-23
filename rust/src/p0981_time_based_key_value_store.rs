/*
 * 981. Time Based Key-Value Store
Medium

Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.

Implement the TimeMap class:

    TimeMap() Initializes the object of the data structure.
    void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
    String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".

 

Example 1:

Input
["TimeMap", "set", "get", "get", "set", "get", "get"]
[[], ["foo", "bar", 1], ["foo", 1], ["foo", 3], ["foo", "bar2", 4], ["foo", 4], ["foo", 5]]
Output
[null, null, "bar", "bar", null, "bar2", "bar2"]

Explanation
TimeMap timeMap = new TimeMap();
timeMap.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
timeMap.get("foo", 1);         // return "bar"
timeMap.get("foo", 3);         // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
timeMap.set("foo", "bar2", 4); // store the key "foo" and value "bar2" along with timestamp = 4.
timeMap.get("foo", 4);         // return "bar2"
timeMap.get("foo", 5);         // return "bar2"

 

Constraints:

    1 <= key.length, value.length <= 100
    key and value consist of lowercase English letters and digits.
    1 <= timestamp <= 107
    All the timestamps timestamp of set are strictly increasing.
    At most 2 * 105 calls will be made to set and get.
 */

use std::collections::HashMap;

pub struct TimeMap<'a> {
    pub map: HashMap<&'a str, Vec<(&'a str, i32)>>,
}


impl<'a> TimeMap<'a> {

    pub fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    
    pub fn set(&mut self, key: &'a str, value: &'a str, timestamp: i32) {
        // if key not found, push to newly created list
        self.map.entry(key).or_insert(vec![]).push((value, timestamp));
    }
    
    pub fn get(&self, key: &str, timestamp: i32) -> &str {
        let mut result: &str = "";

        // if not found return empty string slice
        if !self.map.contains_key(key) {
            return result 
        }

        // pull out list to do binary search on, quicker than O(n) scan
        // giving O(log n)
        let list = self.map.get(key).unwrap();
        let (mut l, mut r) = (0, list.len()-1);
        let mut mid: usize;


        // Trivia: When deciding between < and <=, for binary search,
        // e.g. odd, 1 element list [x] needs <= instead of <, (0+0)/2 = 0,
        while l <= r {
            mid = ((l+r)/2) as usize;
            if list[mid].1 == timestamp {
                return list[mid].0
            }

            // if greater go left
            if list[mid].1 > timestamp {
                r = mid - 1;
            } else {
                // if smaller, capture list[mid] value
                // as this may be the next smallest value if value not found
                // and proceed right
                result = list[mid].0;
                l = mid + 1;
            }
        }

        result
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0981() {
        let mut tm = TimeMap::new();
        tm.set("foo", "bar", 1);  // store the key "foo" and value "bar" along with timestamp = 1.
        assert_eq!("bar", tm.get("foo", 1));   // return "bar"
        assert_eq!("bar", tm.get("foo", 3));   // return "bar", since there is no value corresponding to foo at timestamp 3 and timestamp 2, then the only value is at timestamp 1 is "bar".
        tm.set("foo", "bar2", 4);              // store the key "foo" and value "bar2" along with timestamp = 4.
        assert_eq!("bar2", tm.get("foo", 4));  // return "bar2"
        assert_eq!("bar2", tm.get("foo", 5));  // return "bar2"
    }
}

