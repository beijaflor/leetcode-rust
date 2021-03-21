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


fn get_val(s: &str) -> (i32, &str, bool) {
    let mut closed = false;
    let pos = s.chars().position(|v| {
        if v == '(' {
            true
        } else if v == ')' {
            closed = true;
            true
        } else {
            false        
        }
    }).unwrap();
    (s[..pos].parse().unwrap(), &s[pos..], closed)
}

fn get_children(s: &str) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>, &str) {
    let (val,  s, closed) = get_val(s);
    println!("val: {}, s: {:?}, closed: {}", val, s, closed);
    if closed {
        let pos = s.chars().position(|v| v != ')' && v != '(');

        match pos {
            None => {
                println!("None");
                (
                    Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                    None,
                    "",
                )
            },
            Some(pos) => {
                let (left, right, s) = get_children(&s[pos..]);
                (
                    Some(Rc::new(RefCell::new(TreeNode {
                        val: val,
                        left: left,
                        right: right,
                    }))),
                    None,
                    s,
                )
            },
        }
    } else {
        let (left, right, s) = get_children(&s[1..]);
        let pos = s.chars().position(|v| v != ')' && v != '(');
        // println!("{:?}", pos);
        let right = match pos {
            None => None,
            Some(pos) => get_children(&s[pos..]).0,
        };

        (
            Some(Rc::new(RefCell::new(TreeNode {
                val: val,
                left: left,
                right: right,
            }))),
            None,
            s,
        )
    }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let (result, _, _) = get_children(&s);
        println!("RESULT: {:?}\n\n", result);
        result
    }
}
