/*
 * 892 · Alien Dictionary

There is a new alien language which uses the latin alphabet. However, the order among letters are unknown to you. You receive a list of non-empty words from the dictionary, where words are sorted lexicographically by the rules of this new language. Derive the order of letters in this language.


    You may assume all letters are in lowercase.
    The dictionary is invalid, if string a is prefix of string b and b is appear before a.
    If the order is invalid, return an empty string.
    There may be multiple valid order of letters, return the smallest in normal lexicographical order.
    The letters in one string are of the same rank by default and are sorted in Human dictionary order.

Example

Example 1:

Input：["wrt","wrf","er","ett","rftt"]

Output："wertf"

Explanation：

from "wrt"and"wrf" ,we can get 't'<'f'

from "wrt"and"er" ,we can get 'w'<'e'

from "er"and"ett" ,we can get 'r'<'t'

from "ett"and"rftt" ,we can get 'e'<'r'

So return "wertf"

Example 2:

Input：["z","x"]

Output："zx"

Explanation：

from "z" and "x"，we can get 'z' < 'x'

So return "zx"
 *
 */


/*
 * Thoughts
 *
 * Given a words (lexicographically) sorted list like so:
 * ["wrt","wrf","er","ett","rftt"] 
 *
 * For each word pair, e.g word1 and word2, word 2 and word 3, etc..
 * Build the graph adjacent list for characters basically using the hint:
 
   from "wrt"and"wrf" ,we can get 't'<'f'
   from "wrt"and"er" ,we can get 'w'<'e'
   from "er"and"ett" ,we can get 'r'<'t'
   from "ett"and"rftt" ,we can get 'e'<'r'

   Hence we know: 

   t has directed edge to f: t -> f
   w has directed edge to e: w -> e
   r has directed edge to t: r -> t
   e has directed edge to r: e -> r

   Also keep track of how many other nodes are pointing to a node via indegrees

   With this graph and indegrees info, grab those nodes who have indegrees of 0 (nobody preceding them) and seed the bfs queue

   Process each of these indegree 0 nodes, by first adding them to the output, decrementing their directed edge peer nodes indegrees
   (since they no longer be pointed at from this node since now processed already), checking if their peer node indegrees have fallen to 0
   and then adding that to the bfs queue

   Keep doing this

   If there is a cycle than two or more nodes will never have their indegrees go down to 0 and thus would be included for bfs processing

   Hence at end we ensure that length of String result is the same as number of graph character keys
   Because if there is a cycle the String result len will be smaller than the graph character keys
 */

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let error = "".to_string();
        let mut zipped_pairs;
        let mut result = String::new();

        // graph tracks the directed edges coming out of the node (char)
        // indegrees tracks the directed edges count coming into a node (char)

        /* Imperative style
        let mut graph: HashMap<char, HashSet<char>> = HashMap::new();
        for i in 0..words.len() {
            let chars = words[i].chars();
            for j in chars {
                graph.entry(j).or_insert_with(|| HashSet::new());
            }
        }
        */

        /* Functional style */
        let mut graph: HashMap<char, HashSet<char>> =
            words.iter().flat_map(|s| s.chars()).fold(HashMap::new(), |mut acc, c| {
                acc.entry(c).or_insert_with(|| HashSet::new());
                acc
            });

        let mut indegrees: HashMap<char, u32> = HashMap::new();

        let (mut w1, mut w2): (&String, &String);

        for i in 0..words.len()-1 {
            w1 = &words[i];
            w2 = &words[i+1];

            let minlen = std::cmp::min(w1.len(), w2.len());

            // e.g. if we have "ert" and "er" then return error as order is inconsistent 
            if w1.len() > w2.len() && &w1[..minlen] == &w2[..minlen] {
                return error
            }

            zipped_pairs = w1.chars().zip(w2.chars());

            // The imperative way would have been for i in 0..minlen { if w1[i] != w2[i]} where w1/w2 would have to be a char vec slice..

            // Compare the word pairs and construct the graph and indegrees structures
            while let Some((a,b)) = zipped_pairs.next() {
                if a != b {
                    // mark that a now has a directed edge to b
                    graph.entry(a).or_insert_with(|| HashSet::new()).insert(b);

                    // mark that b has someone pointing into it via a directed edge
                    // mark that a is in the indegrees map with 0 if not seen yet
                    *indegrees.entry(b).or_insert(0) += 1;
                    indegrees.entry(a).or_insert(0);

                    // Don't need to continue evaluating since at this point the words are not the same
                    break;
                }
            }
        }

        // Note cycles on their own e.g. z -> x -> z will never have indegrees 0 because z will always be 1, and x will always be 1
        // Seed the ready to process queue
        let mut queue = indegrees.iter().fold(VecDeque::new(), |mut acc, (k,v)| {
            if v == &0 {acc.push_back(*k)};
            acc
        });

        // Process the queue bfs style
        let mut node: char;
        while !queue.is_empty() {
            node = queue.pop_front().unwrap();

            // add char to resultant String
            result.push(node);

            if let Some(directed_peers) = graph.get(&node) {
                for p in directed_peers {
                    // add first if count is 1 since we'll decrement it later
                    // a node's indegree never increases so the node won't be processed again
                    if let Some(&1) = indegrees.get(&p) { queue.push_back(*p) }
                    // reduce indegree for peer since the node that is pointing to it has been processed
                    indegrees.entry(*p).and_modify(|e| if *e > 0 {*e -= 1});
                }
            }
        }

        // the number of chars in result should be the same size as the char keys in graph
        if result.len() == graph.len() { result } else { error }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0892() {
        let input: Vec<String> = vec!["wrt","wrf","er","ett","rftt"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!("wertf".to_string(), Solution::alien_order(input));
        assert_eq!("zx".to_string(), Solution::alien_order(vec!["z".to_string(), "x".to_string()]));
        assert_eq!("".to_string(), Solution::alien_order(vec!["z".to_string(), "x".to_string(), "z".to_string()]));
    }
}

/*
running 1 test
[src/p0892_alien_dictionary.rs:133] &graph = {
    'w': {
        'e',
    },
    'f': {},
    'r': {
        't',
    },
    'e': {
        'r',
    },
    't': {
        'f',
    },
}
[src/p0892_alien_dictionary.rs:134] &indegrees = {
    't': 1,
    'f': 1,
    'e': 1,
    'w': 0,
    'r': 1,
}
[src/p0892_alien_dictionary.rs:150] &result = "w"
[src/p0892_alien_dictionary.rs:150] &result = "we"
[src/p0892_alien_dictionary.rs:150] &result = "wer"
[src/p0892_alien_dictionary.rs:150] &result = "wert"
[src/p0892_alien_dictionary.rs:150] &result = "wertf"
[src/p0892_alien_dictionary.rs:162] &graph = {
    'w': {
        'e',
    },
    'f': {},
    'r': {
        't',
    },
    'e': {
        'r',
    },
    't': {
        'f',
    },
}
[src/p0892_alien_dictionary.rs:163] &indegrees = {
    't': 0,
    'f': 0,
    'e': 0,
    'w': 0,
    'r': 0,
}
[src/p0892_alien_dictionary.rs:164] &result = "wertf"








[src/p0892_alien_dictionary.rs:133] &graph = {
    'x': {},
    'z': {
        'x',
    },
}
[src/p0892_alien_dictionary.rs:134] &indegrees = {
    'z': 0,
    'x': 1,
}
[src/p0892_alien_dictionary.rs:150] &result = "z"
[src/p0892_alien_dictionary.rs:150] &result = "zx"
[src/p0892_alien_dictionary.rs:162] &graph = {
    'x': {},
    'z': {
        'x',
    },
}
[src/p0892_alien_dictionary.rs:163] &indegrees = {
    'z': 0,
    'x': 0,
}
[src/p0892_alien_dictionary.rs:164] &result = "zx"





[src/p0892_alien_dictionary.rs:133] &graph = {
    'z': {
        'x',
    },
    'x': {
        'z',
    },
}
[src/p0892_alien_dictionary.rs:134] &indegrees = {
    'x': 1,
    'z': 1,
}
[src/p0892_alien_dictionary.rs:162] &graph = {
    'z': {
        'x',
    },
    'x': {
        'z',
    },
}
[src/p0892_alien_dictionary.rs:163] &indegrees = {
    'x': 1,
    'z': 1,
}
[src/p0892_alien_dictionary.rs:164] &result = ""
    test p0892_alien_dictionary::tests::test_0892 ... ok

    */
