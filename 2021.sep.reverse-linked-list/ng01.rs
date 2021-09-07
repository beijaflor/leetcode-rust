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

fn dig(node: &Box<ListNode>, parent: Option<&Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(child) = &node.next {
        if let Some(mut rev_parent) = dig(&child, Some(node)) {
            rev_parent.next = if parent != None { Some(Box::new(*parent.unwrap().clone())) } else { None };
            println!("{:?}", child);
            println!("1, {:?}", rev_parent);
            Some(rev_parent)
        } else {
            println!("2");
            Some(Box::new(*child.clone()))
        }
    } else {
        None
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(node) = head {
            dig(&node, None)
        } else {
            None
        }     
    }
}