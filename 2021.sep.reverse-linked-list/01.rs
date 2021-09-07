// https://leetcode.com/submissions/detail/550890418/
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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None { return None }
        let mut vec = node2vec(head);
        vec.reverse();
        let mut len = vec.len() - 1;
        vec2node(&vec, None, len)
    }
}