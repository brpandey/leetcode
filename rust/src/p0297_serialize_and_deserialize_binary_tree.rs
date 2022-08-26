/*
 * 297. Serialize and Deserialize Binary Tree
Hard

Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.

Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.

Clarification: The input/output format is the same as how LeetCode serializes a binary tree. You do not necessarily need to follow this format, so please be creative and come up with different approaches yourself.

 

Example 1:

Input: root = [1,2,3,null,null,4,5]
Output: [1,2,3,null,null,4,5]

Example 2:

Input: root = []
Output: []

 

Constraints:

    The number of nodes in the tree is in the range [0, 104].
    -1000 <= Node.val <= 1000


 *
 *
 */


use crate::util::TreeNode;
use crate::util::TreeNodeRef;

use std::rc::Rc;
use std::cell::RefCell;

struct Codec {}

#[allow(dead_code)]
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    // convert nodes to a string containing the values separated by commas
    // null is marked as !, using a dfs preorder (NLR) approach
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result = String::with_capacity(20);
        Self::dfs1(root, &mut result);

        result
    }

    fn dfs1(node: Option<TreeNodeRef>, output: &mut String) {
        // Base case root is None
        if node == None {
            output.push_str("!,");
            return
        }

        let val = TreeNode::value(&node).to_string();

        output.push_str(&val);
        output.push_str(",");

        Self::dfs1(TreeNode::left(&node), output);
        Self::dfs1(TreeNode::right(&node), output);
    }

    // Split data up into elements utilizing comma delim
    // Pull root node off, and build rest of the subtrees recursively
    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let input: Vec<&str> = data.split_terminator(",").collect();

        if "!" == input[0] {
            return None
        }

        let mut index: usize = 0;

        Self::dfs2(&mut index, &input)
    }

    fn dfs2(index: &mut usize, input: &[&str]) -> Option<TreeNodeRef> {
        // do DFS search using preorder traversal (NLR) to build up sub trees

        if input[*index] == "!" {
            *index += 1;
            return None
        }

        let val: i32 = input[*index].parse().unwrap();
        let node = TreeNode::new(val);

        *index += 1;

        node.as_ref().borrow_mut().left = Self::dfs2(index, input);
        node.as_ref().borrow_mut().right = Self::dfs2(index, input);

        return Some(node);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_0297() {
        let c = Codec::new();
        let root = TreeNode::from_list(vec![1,2,3,i32::MIN,i32::MIN,4,5]);
        let root2 = root.as_ref().map(Rc::clone);
        let output = c.deserialize(c.serialize(root2));
        assert_eq!(root, output);

        assert_eq!(None, c.deserialize(c.serialize(None)));
    }
}

/*
&output = Some(
    RefCell {
        value: TreeNode {
            val: 1,
            left: Some(
                RefCell {
                    value: TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    },
                },
            ),
            right: Some(
                RefCell {
                    value: TreeNode {
                        val: 3,
                        left: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 4,
                                    left: None,
                                    right: None,
                                },
                            },
                        ),
                        right: Some(
                            RefCell {
                                value: TreeNode {
                                    val: 5,
                                    left: None,
                                    right: None,
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
