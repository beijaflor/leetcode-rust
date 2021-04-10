/*
[6,2,8,0,4,7,9,null,null,3,5]
2
[2,1,3]
1
[2,1]
2
[2,null,3]
2
[5,3,6,2,4,null,null,1]
6
[5,3,6,2,4,null,null,1]
3
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

fn dig(node: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(right) = node.borrow().right.clone() {
        return dig(right.clone())
    }
    Some(node)
}

fn trace(root: Rc<RefCell<TreeNode>>, p: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root == p {
        return dig(p)
    }
    let target_val = p.borrow().val;
    let root_val = root.borrow().val;

    let result = if target_val < root_val {
        if let Some(left_node) = root.clone().borrow().left.clone() {
            let result = trace(left_node, p.clone());
            if result == Some(p) {
                Some(root)
            } else {
                result
            }
        } else {
            Some(root)
        }
    } else {
        if let Some(right_node) = root.borrow().right.clone() {
            trace(right_node, p)
        } else {
            None
        }
    };
    result
}

impl Solution {
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_node = if let Some(node) = root { node } else { return None };
        let target_node = if let Some(node) = p.clone() { node } else { return None };
        let result = trace(root_node, target_node);
        if result == p {
            None
        } else {
            result
        }
    }
}