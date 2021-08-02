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

fn btree2vec(node: Option<&Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
  let node = node.unwrap();
  let mut result: Vec<Vec<i32>> = vec![];
  if node.borrow().left == None && node.borrow().right == None {
      return vec![vec![node.borrow().val]]
  } else {
      let mut left = if node.borrow().left != None {
          btree2vec(node.borrow().left.as_ref())
      } else { vec![] };

      let mut right = if node.borrow().right != None {
          btree2vec(node.borrow().right.as_ref())
      } else { vec![] };
      
      for lindex in 0..left.len() {
          for rindex in 0..right.len() {
              if left[lindex].last().unwrap() + 1 == *right[rindex].first().unwrap() {
                  let vec = left[lindex].clone().into_iter().chain(right[rindex].clone().into_iter()).collect();
                  result.push(vec);
              }
          }
      }

      result.append(&mut right);
      result.append(&mut left);

      result = result.into_iter().map(|mut vec| {
          if node.borrow().val + 1 == *vec.first().unwrap() {
              vec.insert(0, node.borrow().val);
          } else if node.borrow().val - 1 == *vec.last().unwrap() {
              vec.push(node.borrow().val);
          }
          vec
      }).collect();
  }
  result
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let mut r = btree2vec(root.as_ref());
      println!("{:?}", r);
      r.sort_by(|lhv, rhv| lhv.len().cmp(&rhv.len()));
      
      r.last().unwrap().len() as i32
  }
}