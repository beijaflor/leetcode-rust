// https://leetcode.com/submissions/detail/480808842/
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

fn append_last(head: &mut Option<Box<ListNode>>, child: Option<Box<ListNode>>) {
    let mut current = head;
    loop {
        if let Some(node) = current {
            if node.next == None {
                node.next = child;
                break
            } else {
                current = &mut node.next;
            }
        } else {
            break
        }
    }
}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut smaller_node: Option<Box<ListNode>> = None;
        let mut greater_node: Option<Box<ListNode>> = None;
        let mut current = head;    
        while let Some(mut node) = current {
            current = node.next;
            node.next = None;
            if node.val < x {
                if smaller_node == None {
                    smaller_node = Some(node);
                } else {
                    append_last(&mut smaller_node, Some(node));
                }
            } else {
                if greater_node == None {
                    greater_node = Some(node);
                } else {
                    append_last(&mut greater_node, Some(node));
                }
            }
        }
        // println!("lt: {:?}", smaller_node);
        // println!("gt: {:?}", greater_node);
        if smaller_node == None {
            greater_node            
        } else {
            append_last(&mut smaller_node, greater_node);
            smaller_node
        }
    }
}