// https://leetcode.com/submissions/detail/610191543/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, max: i32, min: i32) -> i32 {
    // println!("{:?} {} {}", node, max, min);
    if let Some(node) = node {
        let max = i32::max(max, node.borrow().val);
        let min = i32::min(min, node.borrow().val);
        i32::max(max - min, i32::max(dig(node.borrow().left.as_ref(), max, min), dig(node.borrow().right.as_ref(), max, min)))
    } else {
        0
    }
}

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dig(root.as_ref(), i32::MIN, i32::MAX)
    }
}