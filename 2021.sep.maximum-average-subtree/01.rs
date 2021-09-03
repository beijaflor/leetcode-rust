// https://leetcode.com/submissions/detail/548348358/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32, f64) {
    if let Some(node) = node {
        let val = node.borrow().val;
        let (lcount, ltotal, lmax) = dig(node.borrow().left.as_ref());
        let (rcount, rtotal, rmax) = dig(node.borrow().right.as_ref());
        let count = lcount + rcount + 1;
        let total = ltotal + rtotal + val;
        let average = total as f64 / count as f64;
        let max = f64::max(f64::max(lmax, rmax), average);
        (count, total, max)
    } else {
        (0, 0, 0.0)
    }
}


impl Solution {
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        dig(root.as_ref()).2
    }
}