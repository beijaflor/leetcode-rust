// https://leetcode.com/submissions/detail/494187329/
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

#[derive(Debug, PartialEq)]
enum Seen {
    OnCamera,
    SeenByCamera,
    UnSeen,
    Leaf,
    End,
}


fn dig(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Seen) {
    match node {
        None => (0, Seen::End),
        Some(node) => {
            let (left_camera, left_seen) = dig(node.borrow().left.clone());
            let (right_camera, right_seen) = dig(node.borrow().right.clone());
            // println!("left: {:?}, right: {:?}", left_seen, right_seen);
            
            let isLeaf = (left_seen == Seen::End && right_seen == Seen::End);
            let needsCamera = (left_seen == Seen::Leaf || left_seen == Seen::UnSeen || right_seen == Seen::Leaf || right_seen == Seen::UnSeen);
            let isSeen = (left_seen == Seen::OnCamera || right_seen == Seen::OnCamera);

            if isLeaf {
                (0, Seen::Leaf)
            } else
            if needsCamera {
                (left_camera + right_camera + 1, Seen::OnCamera)
            } else
            if !isSeen {
                (left_camera + right_camera, Seen::UnSeen)
            } else {
                (left_camera + right_camera, Seen::SeenByCamera)
            }
        }
    }    
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (count, seen) = dig(root);
        // println!("result: {:?}, {:?}\n", count, seen);
        if seen == Seen::Leaf || seen == Seen::UnSeen {
            count + 1
        } else {
            count
        }
    }
}