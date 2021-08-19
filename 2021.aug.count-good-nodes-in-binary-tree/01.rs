// https://leetcode.com/submissions/detail/539859321/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

fn dfs(root: &Rc<RefCell<TreeNode>>, max: i32) -> i32 {
    let val = root.borrow().val;
    let mut result = 0;
    let next_max = if val >= max {
        result += 1;
        val
    } else {
        max
    };
    if let Some(left) = root.borrow().left.as_ref() {
        result += dfs(left, next_max);
    }
    if let Some(right) = root.borrow().right.as_ref() {
        result += dfs(right, next_max);
    }
    result
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node = root.unwrap();
        let val = node.borrow().val;
        let left = if let Some(left) = node.borrow().left.as_ref() { dfs(left, val) } else { 0 };
        let right = if let Some(right) = node.borrow().right.as_ref() { dfs(right, val) } else { 0 };
        1 + left + right
    }
}