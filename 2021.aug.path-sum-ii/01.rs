// https://leetcode.com/submissions/detail/533165963/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, current: i32) -> Option<Vec<Vec<i32>>> {
    if let Some(node) = node {
        let current = current - node.borrow().val;
        if node.borrow().left == None && node.borrow().right == None {
            if current == 0 {
                return Some(vec![vec![node.borrow().val]])
            } else {
                None
            }
        } else {
            let mut result: Vec<Vec<i32>> = vec![];
            if let Some(mut left_result) = dig(node.borrow().left.as_ref(), target_sum, current) {
                println!("left: {:?}", left_result);
                result.append(&mut left_result);
            };
            if let Some(mut right_result) = dig(node.borrow().right.as_ref(), target_sum, current) {
                println!("right: {:?}", right_result);
                result.append(&mut right_result);
            };
            if result.is_empty() {
                None
            } else {
                Some(result.into_iter().map(|mut vec| {
                    vec.insert(0, node.borrow().val);
                    vec
                }).collect())
            }
        }
    } else {
        None
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(result) = dig(root.as_ref(), target_sum, target_sum) {
            result
        } else {
            vec![]
        }
    }
}