/*
 * 211. Design Add and Search Words Data Structure
Medium

Design a data structure that supports adding new words and finding if a string matches any previously added string.

Implement the WordDictionary class:

    WordDictionary() Initializes the object.
    void addWord(word) Adds word to the data structure, it can be matched later.
    bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.

 */

pub struct Solution {}

impl Solution {

}


#[cfg(test)]
pub mod tests {
    //use super::*;
    use crate::util::TrieNode;

    #[test]
    pub fn test_0211() {
        let mut trie = TrieNode::new();
        trie.add_word("bad".to_string());
        trie.add_word("dad".to_string());
        trie.add_word("mad".to_string());
        assert_eq!(false, trie.search("pad".to_string())); // return False
        assert_eq!(true, trie.search("bad".to_string())); // return True
        assert_eq!(true, trie.search(".ad".to_string())); // return True
        assert_eq!(true, trie.search("b..".to_string())); // return True
    }
}

