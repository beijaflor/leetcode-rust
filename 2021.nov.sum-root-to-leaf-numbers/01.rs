// https://leetcode.com/submissions/detail/581454663/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> Vec<i32> {
    if let Some(node) = node {
        let val = val * 10 + node.borrow().val;
        // println!("{}", val);
        if node.borrow().left == None && node.borrow().right == None {
            return vec![val]
        }
        let left = dig(node.borrow().left.as_ref(), val);
        let right = dig(node.borrow().right.as_ref(), val);
        // println!("{:?} {:?}", left, right);
        left.into_iter().chain(right.into_iter()).collect::<Vec<i32>>()
    } else {
        vec![]
    }
}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dig(root.as_ref(), 0).into_iter().sum()
    }
}