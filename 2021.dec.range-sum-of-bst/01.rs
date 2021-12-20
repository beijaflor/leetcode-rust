// https://leetcode.com/submissions/detail/601656321/

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

fn dig(root: Option<&Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node) = root {
        let val = node.borrow().val;
        dig(node.borrow().left.as_ref(), low, high) + dig(node.borrow().right.as_ref(), low, high) + if low <= val && val <= high { val } else { 0 }
    } else {
        0
    }
}

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        dig(root.as_ref(), low, high)
    }
}