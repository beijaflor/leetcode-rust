// https://leetcode.com/submissions/detail/478759840/
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

fn find_smallest(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // println!("find_smallest: {:?}", node);
    match node {
        None => None,
        Some(node) => {
            if node.borrow().left != None {
                find_smallest(node.borrow().left.clone())
            } else {
                Some(node)
            }
        }
    }
}

fn trace(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = if let Some(node) = root { node } else { return None };
    let p = if let Some(node) = p { node } else { return None };

    if root == p {
        // println!("found!!: {:?}", p.borrow().val);
        let find_smallest = find_smallest(p.borrow().right.clone());
        return if find_smallest == None {
            Some(root)
        } else {
            find_smallest
        }
    }

    let target_val = p.borrow().val;
    let root_val = root.borrow().val;

    let result = if target_val < root_val {
        let result = trace(root.borrow().left.clone(), Some(p.clone()));
        if result == Some(p) {
            Some(root)
        } else {
            result
        }
    } else {
        trace(root.borrow().right.clone(), Some(p))
    };
    result
}

impl Solution {
    pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let root_node = if let Some(node) = root.clone() { node } else { return None };
        let target_node = if let Some(node) = p.clone() { node } else { return None };

        if root_node == target_node {
            return find_smallest(root_node.clone().borrow().right.clone())
        }

        let right_result = trace(root_node.clone().borrow().right.clone(), p.clone());
        if let Some(result) = right_result {
            if result == target_node {
                None
            } else {
                Some(result)
            }
        } else {
            let left_result = trace(root_node.clone().borrow().left.clone(), p.clone());
            if let Some(result) = left_result {
                if result == target_node {
                    if result != root_node {
                        root
                    } else {
                        find_smallest(root_node.clone().borrow().right.clone())
                    }
                } else {
                    Some(result)
                }
            } else {
                None
            }
        }
    }
}