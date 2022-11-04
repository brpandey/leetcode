/*
 * 127. Word Ladder
Hard

A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:

    Every adjacent pair of words differs by a single letter.
    Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
    sk == endWord

Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

Example 1:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5
Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.

Example 2:

Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
Output: 0
Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.

Constraints:

    1 <= beginWord.length <= 10
    endWord.length == beginWord.length
    1 <= wordList.length <= 5000
    wordList[i].length == beginWord.length
    beginWord, endWord, and wordList[i] consist of lowercase English letters.
    beginWord != endWord
    All the words in wordList are unique.
 */

/*
 * Strategy
 *
 * Iterate through words creating a map of pattern keys and there word values
 *
 * e.g.
 *
 * pattern key *ot, values: [hot, dot, lot]
 * pattern key h*t, values: []
 * pattern key ho*, values: []
 * pattern key d*t, values: []
 * pattern key do*, values: [dot, dog]
 * pattern key *og, values: [dog, log, cog]
 * and so on ...
 *
 *
 * Then start at begin_word as if was a root node and proceed in bfs search
 * manner until end word is found, returning the number of bfs levels found
 * as each level indicates a single character change since children
 * are 1 character different
 */

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        if word_list.iter().find(|x| **x == end_word).is_none() { return 0 }

        // construct patterns map for each word in word list
        // and for each '*' representation e.g. for word string "abc"
        // produce "*bc", "a*c" ,"ab*"
        let mut patterns: HashMap<Vec<u8>, Vec<&String>> = HashMap::new();
        word_list.push(begin_word.clone());

        for w in &word_list {
            let b = w.as_bytes();
            for i in 0..b.len() {
                let mut bytes: Vec<u8> = b.to_vec();
                bytes[i] = b'*';
                patterns.entry(bytes).or_insert(vec![]).push(w);
            }
        }

//        dbg!(&patterns);

        // setup bfs traverse structures, seeding queue with
        // beginning word
        let mut queue = VecDeque::new();
        queue.push_back(&begin_word);

        let mut visited = HashSet::new();
        visited.insert(&begin_word);
        let mut level_cnt = 1;

        while !queue.is_empty() {
            // process the current level
            for _i in 0..queue.len() {
                let w = queue.pop_front().unwrap();

                // reached path end
                if *w == end_word {
                    return level_cnt;
                }

                // grab child nodes as identified by transforming
                // word into various pattern keys, e.g. for word string "abc"
                // lookup matching words for "*bc", "a*c", "ab*"
                let b = w.as_bytes();
                for i in 0..b.len() {
                    let mut bytes = b.to_vec();
                    bytes[i] = b'*';

                    let children = patterns.get(&bytes).unwrap();

                    for child in children {
                        if !visited.contains(child) {
                            visited.insert(child);
                            queue.push_back(child);
                        }
                    }
                }
            }

            // seen all child level nodes for w, so increment cnt
            level_cnt += 1;
        }

        level_cnt
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0127() {
        let wl1 = vec!["hot", "dot", "dog", "lot", "log", "cog"]; 
        let words1 = wl1.iter().map(|s| s.to_string()).collect();
        assert_eq!(5, Solution::ladder_length("hit".to_string(), "cog".to_string(), words1));

        let wl2 =  vec!["hot", "dot", "dog", "lot", "log"]; 
        let words2 = wl2.iter().map(|s| s.to_string()).collect();
        assert_eq!(0, Solution::ladder_length("hit".to_string(), "cog".to_string(), words2));
    }
}

