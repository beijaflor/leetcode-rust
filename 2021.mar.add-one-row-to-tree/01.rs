// https://leetcode.com/submissions/detail/465927941/
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
use std::collections::VecDeque;

impl Solution {
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, v: i32, d: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::new();
        match root {
            None => None,
            Some(root) => {
                if d == 1 {
                    let mut new_node = TreeNode::new(v);
                    new_node.left = Some(root);
                    return Some(Rc::new(RefCell::new(new_node)))
                }

                let mut depth = 2;
                queue.push_back(root.clone());
                
                while !queue.is_empty() {
                    if d == depth {
                        let mut is_some = false;
                        queue.iter().for_each(|q| {
                            // println!("queue: {:?}", q.borrow().val);

                            let mut new_node = TreeNode::new(v);
                            if q.borrow().left != None {
                                new_node.left = q.borrow().left.clone();
                            }
                            q.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));

                            let mut new_node = TreeNode::new(v);
                            if q.borrow().right != None {
                                new_node.right = q.borrow().right.clone();
                            }
                            q.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                        });
                        break
                    } else {
                        let mut next_queue = vec![];
                        while let Some(q) = queue.pop_front() {
                            // println!("{:?}", q);
                            if let Some(left) = &q.borrow().left {
                                next_queue.push(left.clone());
                            }
                            if let Some(right) = &q.borrow().right {
                                next_queue.push(right.clone());
                            }
                        }
                        next_queue.into_iter().for_each(|q| queue.push_back(q));
                    }
                    depth += 1;
                }
                Some(root)
            }
        }
    }
}