use crate::leetcode::TreeNode;

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                Self::dfs(&node.borrow().left).max(Self::dfs(&node.borrow().right)) + 1
            }
        }
    }
}