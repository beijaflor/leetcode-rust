// https://leetcode.com/submissions/detail/450150717/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn dig(node: &Option<Box<ListNode>>) -> Vec<i32> {
    return match node {
        None => vec![],
        Some(n) => {
            let mut ret = dig(&n.next);
            ret.push(n.val);
            ret
        }
    }
}

fn generate_nodes(mut nodes: Vec<i32>) -> Option<Box<ListNode>> {
    return if nodes.len() == 0 {
        None
    } else {
        let current = nodes.pop().unwrap();
        Some(Box::new(ListNode { val: current, next: generate_nodes(nodes) }))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ret: Vec<i32> = lists.iter().map(|node| dig(node)).collect::<Vec<Vec<i32>>>().into_iter().flatten().collect();
        ret.sort();
        let reverse = ret.into_iter().rev().collect::<Vec<i32>>();
        generate_nodes(reverse)
    }
}