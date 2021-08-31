// https://leetcode.com/submissions/detail/546403254/
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

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, sums: &mut Vec<i32>) -> i32 {
    if let Some(node) = node {
        let sum = node.borrow().val + dig(node.borrow().left.as_ref(), sums) + dig(node.borrow().right.as_ref(), sums);
        sums.push(sum);
        sum
    } else {
        0
    }
}

impl Solution {
    pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut subtree_sums: Vec<i32> = Vec::new();
        
        let total = dig(root.as_ref(), &mut subtree_sums);
        
        // println!("{:?} {:?}", total, subtree_sums);
        
        // remove total
        subtree_sums.pop();
        
        if total.abs() % 2 == 1 {
            false
        } else {
            subtree_sums.contains(&(total / 2))
        }
    }
}