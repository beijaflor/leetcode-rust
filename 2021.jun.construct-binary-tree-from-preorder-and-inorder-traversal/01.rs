// https://leetcode.com/submissions/detail/504997008/
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
use std::collections::HashMap;

struct TreeBuilder {
    index: usize,
    inorderIndexMap: HashMap<i32, usize>,
}

impl TreeBuilder {
    fn new (inorder: Vec<i32>) -> Self {
        let mut map: HashMap<i32, usize> = HashMap::new();
        inorder.into_iter().enumerate().for_each(|(index, val)| {
            map.insert(val, index);
        });
        TreeBuilder {
            index: 0,
            inorderIndexMap: map,
        }
    }
    
    fn arreyToTree(&mut self, preorder: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // println!("left: {}, right: {}", left, right);
        if left > right { return None }

        let val = preorder[self.index];
        // println!("index: {}, val: {}", self.index, val);
        self.index += 1;
        let mut node = TreeNode::new(val);

        node.left = self.arreyToTree(preorder, left, (self.inorderIndexMap.get(&val).unwrap() - 1) as i32);
        node.right = self.arreyToTree(preorder, (self.inorderIndexMap.get(&val).unwrap() + 1) as i32, right);
        
        Some(Rc::new(RefCell::new(node)))
    }
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut tree = TreeBuilder::new(inorder);
        // println!("{:?}", tree.inorderIndexMap);
        let len = (preorder.len() - 1) as i32;
        tree.arreyToTree(&preorder, 0, len)
    }
}