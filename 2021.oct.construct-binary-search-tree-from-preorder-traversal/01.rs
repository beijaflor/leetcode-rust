// https://leetcode.com/submissions/detail/570352445/
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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            None
        } else {
            let val = preorder[0];
            let len = preorder.len();
            let mut node = TreeNode::new(val);
            let pos = preorder.iter().position(|v| *v > val).unwrap_or(len);
            node.left = Solution::bst_from_preorder(preorder[1..pos].to_vec());
            node.right = Solution::bst_from_preorder(preorder[pos..len].to_vec());
            // println!("{}", pos);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}