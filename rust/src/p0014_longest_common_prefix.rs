/*
14. Longest Common Prefix
Easy

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Note:

All given inputs are in lowercase letters a-z.

*/

use std::collections::HashMap;
use std::str::Chars;

// pub struct TrieInfo(usize, usize, usize); // wc, child_count, entries

#[derive(Debug)]
pub struct Trie {
    wc: usize,
    child_count: usize,
    value: Option<char>,
    children: HashMap<char, Box<Trie>>,
    entries: usize,
}

impl Trie {
    pub fn new(letter: Option<char>) -> Trie {
        Trie {
            wc: 0,
            child_count: 0,
            value: letter,
            children: HashMap::new(),
            entries: 0,
        }
    }

    pub fn put(&mut self, mut iter: Chars) {
        let mut current: &mut Trie = self;
        let mut new_child: bool;

        // Rust doesn't have tail recursion so using a while loop instead for very long strings
        while let Some(c) = iter.next() {
            new_child = false;
            let child: &mut Trie = current.children.entry(c).or_insert_with(|| {
                new_child = true;
                Box::new(Trie::new(Some(c)))
            });

            // Indicate we've added another child
            if new_child {
                current.child_count += 1;               
            }

            current.wc += 1;
            current = child;
        }

        current.wc += 1;
    }

    pub fn put_all(&mut self, words: &[&str]) {
        for i in words {
            self.put(i.chars());
            self.entries += 1;
        }
    }

    pub fn longest_prefix(&mut self) -> String {
        let root_entries = self.entries;
        let mut current: &mut Trie = self;
        let mut prefix = String::new();

        while current.wc == root_entries {
            // If not root node
            if let Some(value) = current.value {
                prefix.push(value)
            }
            // If has a single direct descendent, proceed further
            if current.child_count == 1 {
                if let Some(&key) = current.children.keys().next() {
                    current = current.children.get_mut(&key).unwrap();
                }
            } else {
                // no longer have a single common prefix
                break;
            }
        }

        prefix
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0014(){

        let entries1 = &["flower", "flow", "flight"];
        let entries2 = &["flower", "flow", "flows"];
        let entries3 = &["flower", "flower", "flower"];
        let entries4 = &["dog", "racecar", "car"];
        let entries5 = &["global", "glossary", "globe"];


        // See output of entries 1 trie below
        let trie: &mut Trie = &mut Trie::new(None);
        trie.put_all(entries1);
//        println!("trie is {:#?}", trie);
        assert_eq!("fl", trie.longest_prefix());

        let trie: &mut Trie = &mut Trie::new(None);
        trie.put_all(entries2);
        assert_eq!("flow", trie.longest_prefix());

        let trie: &mut Trie = &mut Trie::new(None);
        trie.put_all(entries3);
        assert_eq!("flower", trie.longest_prefix());

        let trie: &mut Trie = &mut Trie::new(None);
        trie.put_all(entries4);
        assert_eq!("", trie.longest_prefix());

        let trie: &mut Trie = &mut Trie::new(None);
        trie.put_all(entries5);
        assert_eq!("glo", trie.longest_prefix());
    }
}


/*
trie is Trie {
    wc: 3,
    child_count: 1,
    value: None,
    children: {
        'f': Trie {
            wc: 3,
            child_count: 1,
            value: Some(
                'f',
            ),
            children: {
                'l': Trie {
                    wc: 3,
                    child_count: 2,
                    value: Some(
                        'l',
                    ),
                    children: {
                        'i': Trie {
                            wc: 1,
                            child_count: 1,
                            value: Some(
                                'i',
                            ),
                            children: {
                                'g': Trie {
                                    wc: 1,
                                    child_count: 1,
                                    value: Some(
                                        'g',
                                    ),
                                    children: {
                                        'h': Trie {
                                            wc: 1,
                                            child_count: 1,
                                            value: Some(
                                                'h',
                                            ),
                                            children: {
                                                't': Trie {
                                                    wc: 1,
                                                    child_count: 0,
                                                    value: Some(
                                                        't',
                                                    ),
                                                    children: {},
                                                    entries: 0,
                                                },
                                            },
                                            entries: 0,
                                        },
                                    },
                                    entries: 0,
                                },
                            },
                            entries: 0,
                        },
                        'o': Trie {
                            wc: 2,
                            child_count: 1,
                            value: Some(
                                'o',
                            ),
                            children: {
                                'w': Trie {
                                    wc: 2,
                                    child_count: 1,
                                    value: Some(
                                        'w',
                                    ),
                                    children: {
                                        'e': Trie {
                                            wc: 1,
                                            child_count: 1,
                                            value: Some(
                                                'e',
                                            ),
                                            children: {
                                                'r': Trie {
                                                    wc: 1,
                                                    child_count: 0,
                                                    value: Some(
                                                        'r',
                                                    ),
                                                    children: {},
                                                    entries: 0,
                                                },
                                            },
                                            entries: 0,
                                        },
                                    },
                                    entries: 0,
                                },
                            },
                            entries: 0,
                        },
                    },
                    entries: 0,
                },
            },
            entries: 0,
        },
    },
    entries: 3,
}

*/
