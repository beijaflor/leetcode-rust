// https://leetcode.com/submissions/detail/533166995/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32, current: i32) -> Vec<Vec<i32>> {
    if let Some(node) = node {
        let current = current - node.borrow().val;
        if node.borrow().left == None && node.borrow().right == None {
            if current == 0 {
                return vec![vec![node.borrow().val]]
            } else {
                vec![]
            }
        } else {
            let mut result: Vec<Vec<i32>> = vec![];
            result.append(&mut dig(node.borrow().left.as_ref(), target_sum, current));
            result.append(&mut dig(node.borrow().right.as_ref(), target_sum, current));
            if result.is_empty() {
                vec![]
            } else {
                result.into_iter().map(|mut vec| {
                    vec.insert(0, node.borrow().val);
                    vec
                }).collect()
            }
        }
    } else {
        vec![]
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        dig(root.as_ref(), target_sum, target_sum)
    }
}