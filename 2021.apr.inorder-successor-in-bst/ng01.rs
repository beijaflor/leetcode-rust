/*
[2,null,3]
2
[2,1,3]
1
[5,3,6,2,4,null,null,1]
6
*/
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

fn trace(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root == p {
        // println!("found!!!!");
        return None
    }
    let target_val = if let Some(target_node) = p.clone() {
        target_node.borrow().val
    } else {
        return None
    };
    let root_val = if let Some(root_node) = root.clone() {
        root_node.borrow().val
    } else {
        return None
    };
    let result = if target_val < root_val {
        // println!("trace left: {:?}", root_val);
        trace(root.clone().unwrap().borrow().left.clone(), p)
    } else {
        // println!("trace right: {:?}", root_val);
        trace(root.clone().unwrap().borrow().right.clone(), p)
    };
    let result = if let Some(result_node) = result {
        if result_node.borrow().val > root_val {
            Some(result_node)
        } else {
            root
        }
    } else {
        root
    };
    if result.clone().unwrap().borrow().val > target_val {
        result
    } else {
        None
    }
}

impl Solution {
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        trace(root, p)
    }
}