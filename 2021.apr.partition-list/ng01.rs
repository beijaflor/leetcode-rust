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

fn part(head: Option<Box<ListNode>>, x: i32) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    match head {
        None => (None, None),
        Some(mut node) => {
            if node.val < x {
                node.next = part(node.next, x).0;
                (Some(node), None)
            } else {
                let mut _self = node.clone();
                _self.next = None;
                println!("_self: {:?}", _self);
                let mut result = part(node.next, x).0;
                append_last(&mut result, Some(_self));
                (result, None)
            }
        }
    }
}

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
        println!("{:?}", head);
        part(head, x).0
    }
}