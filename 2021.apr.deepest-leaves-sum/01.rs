// https://leetcode.com/submissions/detail/479616034/
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

fn dig(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    if let Some(node) = node {
        let mut left = dig(node.borrow().left.clone());
        let mut right = dig(node.borrow().right.clone());
        // println!("left: {:?} / right: {:?}", left, right);
        if left[0].len() > 0 {
            result.append(&mut left);
        }
        if right[0].len() > 0 {
            result.append(&mut right);
        }
        if result.len() == 0 {
            result.push(vec![]);
        }
        result = result.into_iter().map(|mut vec| {
            vec.insert(0, node.borrow().val);
            vec
        }).collect::<Vec<Vec<i32>>>();
    } else {
        result.push(vec![]);
    }
    result
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let node_list = dig(root);
        let max_depth = node_list.iter().fold(0, |acc, node| usize::max(acc, node.len()));
        // println!("{} / {:?}", max_depth, node_list);
        node_list
            .into_iter()
            .filter(|node| node.len() == max_depth)
            .fold(0, |acc, node| acc + node.last().unwrap())
    }
}