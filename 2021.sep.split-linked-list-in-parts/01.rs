// https://leetcode.com/submissions/detail/563187618/
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
fn node2vec(n: Option<Box<ListNode>>) -> Vec<i32> {
    if let Some(node) = n {
        let mut vec = node2vec(node.next);
        vec.insert(0, node.val);
        vec
    } else {
        vec![]
    }
}

fn vec2node(v: &Vec<i32>, index: usize) -> Option<Box<ListNode>> {
    // println!("{:?}", v);
    if index < v.len() {
        let mut node = ListNode::new(v[index]);
        node.next = vec2node(v, index + 1);
        Some(Box::new(node))
    } else {
        None
    }
}

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let nodes = node2vec(head);
        // println!("{:?}", nodes);
        let len = nodes.len() / k as usize;
        // println!("{}", len);
        let mut p = 0;
        (0..k as usize).map(|index| {
            if p >= nodes.len() { None }
            else {
                let len = len + if index < nodes.len() % k as usize { 1 } else { 0 };
                // println!("{} {} {:?}", p, len, nodes[p..p + len].to_vec());
                let r = vec2node(&nodes[p..p + len].to_vec(), 0);
                p += len;
                r
            }
        }).collect()
    }
}