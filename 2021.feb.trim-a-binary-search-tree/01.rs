// https://leetcode.com/submissions/detail/451287116/
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
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                if node.borrow().val < low {
                    if let Some(right_node) = &node.borrow().right {
                        return Solution::trim_bst(Some(Rc::clone(&right_node)), low, high)
                    }
                    return None;
                }
                if node.borrow().val > high {
                    if let Some(left_node) = &node.borrow().left {
                        return Solution::trim_bst(Some(Rc::clone(&left_node)), low, high)
                    }
                    return None;
                }
                let mut result_node = TreeNode::new(node.borrow().val);
                if let Some(right_node) = &node.borrow().right {
                    result_node.right = Solution::trim_bst(Some(Rc::clone(&right_node)), low, high)
                }
                if let Some(left_node) = &node.borrow().left {
                    result_node.left = Solution::trim_bst(Some(Rc::clone(&left_node)), low, high)
                }
                Some(Rc::new(RefCell::new(result_node)))
            }
        }
    }
}