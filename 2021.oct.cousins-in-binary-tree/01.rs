// https://leetcode.com/submissions/detail/573165360/
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

fn find(node: Option<&Rc<RefCell<TreeNode>>>, target: i32, parent: i32, depth: i32) -> Option<(i32, i32)> {
    if let Some(node) = node {
        let val = node.borrow().val;
        if val == target {
            return Some((parent, depth + 1))
        }
        let left = find(node.borrow().left.as_ref(), target, val, depth + 1);
        if left != None {
            return left
        }
        let right = find(node.borrow().right.as_ref(), target, val, depth + 1);
        if right != None {
            return right
        }
    }
    None
}

impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let x_node = find(root.as_ref(), x, -1, 0);
        let y_node = find(root.as_ref(), y, -1, 0);
        if x_node == None || y_node == None {
            false
        } else {
            x_node.unwrap().1 == y_node.unwrap().1 && x_node.unwrap().0 != y_node.unwrap().0
        }
    }
}