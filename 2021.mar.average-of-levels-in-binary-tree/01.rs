// https://leetcode.com/submissions/detail/464040708/
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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root == None { return vec![] }

        let mut queue = VecDeque::new();
        let mut result = vec![];

        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut vals = vec![];
            let mut next_nodes = vec![];
            
            while let Some(q) = queue.pop_front() {
                // println!("q: {:?}", q);
                vals.push(q.borrow().val as i64);
                next_nodes.push(q.borrow().left.clone());
                next_nodes.push(q.borrow().right.clone());
            }

            // println!("vals: {:?}, next_nodes: {:?}", vals, next_nodes);
            while let Some(node) = next_nodes.pop() {
                if node == None { continue }
                queue.push_back(node.unwrap());
            }
            
            result.push(vals.iter().sum::<i64>() as f64 / vals.len() as f64);
        }
    
        result
    }
}