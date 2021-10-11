// https://leetcode.com/submissions/detail/569235284/
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
use std::rc::Rc;
use std::cell::RefCell;

fn longest(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(node) = node {
        let (left_max, left_depth) = longest(node.borrow().left.as_ref());
        let (right_max, right_depth) = longest(node.borrow().right.as_ref());
        let depth = i32::max(left_depth + 1, right_depth + 1);
        let max = i32::max(i32::max(left_max, right_max), left_depth + right_depth + 1);
        (max, depth)
    } else {
        (0, 0)
    }
}

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        longest(root.as_ref()).0 - 1
    }
}