// https://leetcode.com/submissions/detail/512341083/
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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
//         if head.clone().unwrap().next == None {
//             return head
//         }
        
        let mut node_vec = node2vec(head);
        let left_pos = left as usize - 1; // node_vec.iter().position(|n| *n == left).unwrap();
        let right_pos = right as usize - 1; // node_vec.iter().position(|n| *n == right).unwrap();
        (0..((right_pos - left_pos) / 2 + 1)).for_each(|offset| {
            node_vec.swap(left_pos + offset, right_pos - offset);
        });
        // println!("{:?}, {}, {}", node_vec, left_pos, right_pos);
        let len = node_vec.len() - 1;
        let result = vec2node(&node_vec, None, len);
        // println!("{:?}", result);
        result
    }
}