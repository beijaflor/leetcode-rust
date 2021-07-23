// https://leetcode.com/submissions/detail/527008023/
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
impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                // println!("root {} / {:?}", node.borrow().val, node);
                let left = Solution::prune_tree(node.borrow().left.clone());
                let right = Solution::prune_tree(node.borrow().right.clone());
                if left == None && right == None && node.borrow().val == 0 {
                    None
                } else {
                    node.borrow_mut().left = left;
                    node.borrow_mut().right = right;
                    Some(node)
                }
            }
        }
    }
}