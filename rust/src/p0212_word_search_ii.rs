/*
 *
 * 212. Word Search II
Hard

Given an m x n board of characters and a list of strings words, return all words on the board.

Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

 

Example 1:

Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
Output: ["eat","oath"]

Example 2:

Input: board = [["a","b"],["c","d"]], words = ["abcb"]
Output: []

 

Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 12
    board[i][j] is a lowercase English letter.
    1 <= words.length <= 3 * 104
    1 <= words[i].length <= 10
    words[i] consists of lowercase English letters.
    All the strings of words are unique.


 *
 */

/*
 * Strategy:
 * Use trie as our reference
 * While scaning each of the cells of the grid, check if there is a possible path
 * in the trie, if so add to output vector
 */

use std::collections::{HashSet, HashMap};

#[derive(Debug)]
pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub terminal: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            terminal: false,
        }
    }

    pub fn add_word(&mut self, word: &String) {
        let mut node = self;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(|| TrieNode::new())    
        }

        node.terminal = true;
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        for w in &words {
            root.add_word(w);
        }

        let max_rows = board.len() as i32;
        let max_cols = board[0].len() as i32;

        let mut output: Vec<String> = vec![];
        let mut buf: Vec<char> = vec![];

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for r in 0..max_rows {
            for c in 0..max_cols {
                Self::search(&mut root, &board, r, c, 
                     &mut visited, &mut output, &mut buf, 
                     max_rows, max_cols);

                buf.clear();
            }
        }

        output.sort();
        output
    }

    pub fn search(mut node: &mut TrieNode, board: &Vec<Vec<char>>, row: i32, col: i32, 
                 visited: &mut HashSet<(usize, usize)>, output: &mut Vec<String>, 
                 buf: &mut Vec<char>, max_rows: i32, max_cols: i32) {
        
        let r = row as usize;
        let c = col as usize;

        // check our bounds, most importantly don't proceed if trie node doesn't
        // contain board character this means it doesn't contain word in our
        // dictionary trie
        if row < 0 || row == max_rows || col < 0 || col == max_cols 
            || visited.contains(&(r,c)) || !node.children.contains_key(&board[r][c]) {
            return
        }

        // 1) update buf
        // 2) updated visited so same position is not re-visited
        // 3) iterate node to child node that matches board[r][c]
        buf.push(board[r][c]);
        visited.insert((r,c));
        node = node.children.get_mut(&board[r][c]).unwrap();

        // if end of word, add to output and return successfully
        if node.terminal {
            let s: String = buf.iter().collect();
            output.push(s);
        }

        // search the four directions: N, S, W, E
        Self::search(node, board, row + 1, col, visited, output, buf, max_rows, max_cols);
        Self::search(node, board, row - 1, col, visited, output, buf, max_rows, max_cols);
        Self::search(node, board, row, col - 1, visited, output, buf, max_rows, max_cols);
        Self::search(node, board, row, col + 1, visited, output, buf, max_rows, max_cols);

        visited.remove(&(r,c));
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_0212() {
        let result = Solution::find_words(vec![
                             vec!['o','a','a','n'],
                             vec!['e','t','a','e'],
                             vec!['i','h','k','r'],
                             vec!['i','f','l','v']],
                             vec!["oath".to_string(),"pea".to_string(),
                             "eat".to_string(),"rain".to_string(), "oathie".to_string()]);
    
        // added oathie as an extra word
        assert_eq!(result, vec!["eat".to_string(), "oath".to_string(), "oathie".to_string()]);
    }
}

