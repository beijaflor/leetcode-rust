// https://leetcode.com/submissions/detail/581763968/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    if let Some(node) = node {
        if node.borrow().left == None && node.borrow().right == None {
            (node.borrow().val, true)
        } else {
            let (left_val, _) = dig(node.borrow().left.as_ref());
            let (right_val, right_is_leaf) = dig(node.borrow().right.as_ref());
            if right_is_leaf {
                (left_val, false)
            } else {
                (left_val + right_val, false)
            }
        }
    } else {
        (0, false)
    }
}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let (r, is_leaf) = dig(root.as_ref()) {
            if is_leaf { 0 } else { r }
        } else {
            0
        }
    }
}