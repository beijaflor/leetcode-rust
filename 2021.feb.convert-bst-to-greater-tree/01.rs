// https://leetcode.com/submissions/detail/454049695/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, offset: i32) -> i32 {
    if node == None { return offset }
    let node = node.unwrap();
    let val = node.borrow().val + dig(node.borrow().right.as_ref(), offset);
    let left_val = dig(node.borrow().left.as_ref(), val);
    node.borrow_mut().val = val;
    if left_val == 0 { val } else { left_val }
}

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref root) = root {
            println!("{}", dig(Some(&Rc::clone(root)), 0));
        }
        root
    }
}