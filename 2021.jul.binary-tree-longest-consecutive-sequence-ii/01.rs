// https://leetcode.com/submissions/detail/530949902/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    match node {
        None => (0, 0, 0),
        Some(node) => {
            let mut increase = 1;
            let mut decrease = 1;
            let mut maxval = 1;
            
            if node.borrow().left != None {
                let (left_increase, left_decrease, left_maxval) = dig(node.borrow().left.as_ref());
                if node.borrow().val == node.borrow().left.as_ref().unwrap().borrow().val + 1 {
                    decrease = left_decrease + 1;
                } else if node.borrow().val == node.borrow().left.as_ref().unwrap().borrow().val - 1 {
                    increase = left_increase + 1;
                }
                maxval = i32::max(maxval, left_maxval);
            }

            if node.borrow().right != None {
                let (right_increase, right_decrease, right_maxval) = dig(node.borrow().right.as_ref());
                if node.borrow().val == node.borrow().right.as_ref().unwrap().borrow().val + 1 {
                    decrease = i32::max(decrease, right_decrease + 1);
                } else if node.borrow().val == node.borrow().right.as_ref().unwrap().borrow().val - 1 {
                    increase = i32::max(increase, right_increase + 1);
                }
                maxval = i32::max(maxval, right_maxval);
            }

            maxval = i32::max(maxval, increase + decrease - 1);
            (increase, decrease, maxval)
        },
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dig(root.as_ref()).2
    }
}