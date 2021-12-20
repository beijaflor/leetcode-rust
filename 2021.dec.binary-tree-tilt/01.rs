// https://leetcode.com/submissions/detail/598648063/
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
fn dig(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    if let Some(node) = node {
        let (left_sum, left_tilt_sum) = dig(node.borrow().left.as_ref());
        let (right_sum, right_tilt_sum) = dig(node.borrow().right.as_ref());
        let tilt = i32::abs(left_sum - right_sum);
        (node.borrow().val + left_sum + right_sum, tilt + left_tilt_sum + right_tilt_sum)
    } else {
        (0, 0)
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dig(root.as_ref()).1
    }
}