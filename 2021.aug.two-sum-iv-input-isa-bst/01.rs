// https://leetcode.com/submissions/detail/542932363/
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
use std::collections::HashSet;

fn dig(node: Option<&Rc<RefCell<TreeNode>>>, k: i32, map: &mut HashSet<i32>) -> bool {
    if let Some(node) = node {
        let val = node.borrow().val;
        if let Some(_) = map.get(&(k - val)) {
            return true
        }
        map.insert(val);
        dig(node.borrow().left.as_ref(), k, map) || dig(node.borrow().right.as_ref(), k, map) 
    } else {
        false
    }
}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut map: HashSet<i32> = HashSet::new();
        dig(root.as_ref(), k, &mut map)
    }
}