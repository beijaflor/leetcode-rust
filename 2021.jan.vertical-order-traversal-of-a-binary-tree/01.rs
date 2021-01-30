// https://leetcode.com/submissions/detail/449546601/
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
use std::collections::BTreeMap;

fn dig(current: Rc<RefCell<TreeNode>>, store: &mut Vec<(i32, i32, i32)>, x: i32, depth: i32) {
    store.push((x, depth, current.borrow().val));

    if let Some(left) = &current.borrow().left {
        dig(Rc::clone(left), store, x - 1, depth - 1);
    };

    if let Some(right) = &current.borrow().right {
        dig(Rc::clone(right), store, x + 1, depth - 1);
    };
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let current = root.unwrap();
        let mut stack: Vec<(i32, i32, i32)> = vec![];
        dig(current, &mut stack, 0, 0);
        // println!("{:?}", stack);

        let mut map: BTreeMap<i32, Vec<(i32, i32)>> = BTreeMap::new();
        stack.iter().for_each(|(x, depth, val)| {
            map.entry(*x).and_modify(|vec| vec.push((*depth, *val))).or_insert(vec![(*depth, *val)]);
        });

        let mut result: Vec<Vec<i32>> = vec![];
        map.into_iter().for_each(|(_, mut vec)| {
            let mut map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
            vec.sort_by(|a, b| b.0.cmp(&a.0));
            vec.iter().for_each(|(depth, val)| {
                map.entry(*depth).and_modify(|vec| vec.push(*val)).or_insert(vec![*val]);
            });
            // println!("{:?}", map);

            let result_partial = map.into_iter().rev().fold(vec![], |mut acc, (_, mut vec)| {
                vec.sort();
                vec.iter().for_each(|v| acc.push(*v));
                acc
            });
            // println!("{:?}", result_partial);
            result.push(result_partial);
        });
        
        // println!("{:?}", result);
        result
    }
}