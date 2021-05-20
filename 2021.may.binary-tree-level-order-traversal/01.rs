// https://leetcode.com/submissions/detail/495909912/
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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut result: Vec<Vec<i32>> = vec![];
        match root {
            None => vec![],
            Some(root) => {
                queue.push_back(root);
                while !queue.is_empty() {
                    let mut level_traversal: Vec<i32> = vec![];
                    let mut new_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
                    while let Some(node) = queue.pop_front() {
                        level_traversal.push(node.borrow().val);
                        if let Some(left) = node.borrow().left.clone() {
                            new_queue.push_back(left);
                        }
                        if let Some(right) = node.borrow().right.clone() {
                            new_queue.push_back(right);
                        }
                    }
                    result.push(level_traversal);
                    queue = new_queue;
                }
                result
            }
        }
    }
}