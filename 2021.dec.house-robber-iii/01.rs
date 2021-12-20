// https://leetcode.com/submissions/detail/597866820/
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

fn dig(node: &Rc<RefCell<TreeNode>>) -> (i32, i32) {
    if node.borrow().left == None && node.borrow().right == None {
        (0, node.borrow().val)
    } else {
        let left = if let Some(left) = &node.borrow().left {
            dig(left)
        } else {
            (0, 0)
        };
        let right = if let Some(right) = &node.borrow().right {
            dig(right)
        } else {
            (0, 0)
        };
        (
            i32::max(left.0, left.1) + i32::max(right.0, right.1),
            left.0 + right.0 + node.borrow().val,
        )
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root.as_ref() {
            let (rob, not) = dig(root);
            i32::max(rob, not)
        } else {
            0
        }
    }
}