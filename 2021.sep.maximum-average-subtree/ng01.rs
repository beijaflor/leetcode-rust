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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, mut v: Vec<i32>) -> f64 {
    if let Some(node) = node {
        v.push(node.borrow().val);
        let left = dig(node.borrow().left.as_ref(), v.clone());
        let right = dig(node.borrow().right.as_ref(), v.clone());
        f64::max(left, right)
    } else {
        let len = v.len();
        v.into_iter().sum::<i32>() as f64 / len as f64
    }
}


impl Solution {
    pub fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
        dig(root.as_ref(), vec![])
    }
}