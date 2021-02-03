// 問題を読み間違えた
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

fn find_node(node: Option<Rc<RefCell<TreeNode>>>, candidate1: i32, candidate2: i32) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
    match node {
        None => (None, candidate1),
        Some(node) => {
            if node.borrow().val == candidate1 {
                return (Some(node.clone()), candidate2)
            }
            if node.borrow().val == candidate2 {
                return (Some(node.clone()), candidate1)
            } else {
                if let Some(left_node) = &node.borrow().left {
                    let result = find_node(Some(Rc::clone(&left_node)), candidate1, candidate2);
                    if let Some(result_node) = result.0 {
                        return (Some(result_node), result.1)
                    }
                }
                if let Some(right_node) = &node.borrow().right {
                    let result = find_node(Some(Rc::clone(&right_node)), candidate1, candidate2);
                    if let Some(result_node) = result.0 {
                        return (Some(result_node), result.1)
                    }
                }
                (None, candidate1)
            }
        }
    }
}

    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let (start_node, end_val) = find_node(root, low, high);
        start_node
    }

fn main() {

    let input = Some(Rc::new(RefCell::new({ TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new({ TreeNode {
            val: 0,
            left: None,
            right: None,
        }}))),
        right: Some(Rc::new(RefCell::new({ TreeNode {
            val: 2,
            left: None,
            right: None,
        }}))),
    }})));
    assert_eq!(
        Some(Rc::new(RefCell::new({ TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new({ TreeNode {
                val: 2,
                left: None,
                right: None,
            }}))),
        }}))),
        trim_bst(input, 1, 2)
    );
    println!("SUCCESS\n\n");
}
