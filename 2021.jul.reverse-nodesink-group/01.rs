// https://leetcode.com/submissions/detail/524525595/
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

fn vec2node(v: &Vec<i32>, child: Option<Box<ListNode>>, index: usize) -> Option<Box<ListNode>> {
    let mut node = ListNode::new(v[index]);
    node.next = child;
    let node = Some(Box::new(node));
    
    if index == 0 {
        node
    } else {
        vec2node(v, node, index - 1)
    }
}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 { return head }
        let mut node_vec = node2vec(head);
        let k = k as usize;
        let mut p = k;
        while p <= node_vec.len() {
            // println!("p: {}", p);
            (0..k / 2).for_each(|offset| {
                node_vec.swap(p - k + offset, p - offset - 1);
            });
            p += k;
        }
        
        
        let len = node_vec.len() - 1;
        let result = vec2node(&node_vec, None, len);
        result
    }
}