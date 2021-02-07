// https://leetcode.com/submissions/detail/452932820/
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
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        if let Some(node) = root {
            queue.push_front(node);
        } else {
            return vec![]
        }
        
        let mut result: Vec<i32> = vec![];

        while !queue.is_empty() {
            let mut tmp: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
            let mut val = 0;
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                val = node.borrow().val;
                if let Some(left) = &node.borrow().left {
                    tmp.push(Rc::clone(left));
                };
                if let Some(right) = &node.borrow().right {
                    tmp.push(Rc::clone(right));
                };
            }
            result.push(val);
            for x in tmp.into_iter() {
                queue.push_back(x);
            }
        }
        result
    }
}