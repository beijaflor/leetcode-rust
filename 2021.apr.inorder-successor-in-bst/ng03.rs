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

fn find_smallest(node: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(left) = node.borrow().left.clone() {
        return find_smallest(left.clone())
    }
    Some(node)
}

fn trace(root: Rc<RefCell<TreeNode>>, p: Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root == p {
        return if let Some(right) = root.clone().borrow().right.clone() {
            find_smallest(right)
        } else {
            Some(root)
        }
    }
    let target_val = p.borrow().val;
    let root_val = root.borrow().val;

    let result = if target_val < root_val {
        if let Some(left_node) = root.clone().borrow().left.clone() {
            trace(left_node, p.clone())
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
        let root_node = if let Some(node) = root.clone() { node } else { return None };
        let target_node = if let Some(node) = p.clone() { node } else { return None };
        let result = trace(root_node, target_node);
        println!("\n\nstart");
        if result == p && result != None {
            if root == p {
                println!("1");
                root
            } else {
                println!("2");
                None            
            }
        } else {
            println!("3");
            result
        }
    }
}