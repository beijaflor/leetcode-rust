use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut dead = HashSet::new();
        deadends.into_iter().for_each(|d| {
            dead.insert(d);
        });
        
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(String::from("0000"));
        queue.push_back(String::from(""));
        
        let mut seen = HashSet::new();
        seen.insert(String::from("0000"));
        
        let mut depth = 0;
        
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if node == String::from("") {
                depth += 1;
                match queue.front() {
                    None => queue.push_back(String::from("")),
                    Some(node) => if node != &String::from("") { queue.push_back(String::from("")) }
                }
            } else if node == target {
                return depth
            } else if !dead.contains(&node) {
                for i in 0..4 {
                    for d in (-1..2).step_by(2) {
                        let y = ((node.chars().nth(i).unwrap() as u8 - '0' as u8) + d as u8 + 10) % 10;
                        let nei = node.chars().enumerate().map(|(index, c)| {
                            if index == i { ('0' as u8 + y) as char } else { c }
                        }).collect::<String>();
                        // println!("y: {}, nei: {}", y, nei);
                        if !seen.contains(&nei) {
                            seen.insert(nei.clone());
                            queue.push_back(nei);
                        }
                    }
                }
            }
        }

        -1
    }
}

//                 for (int i = 0; i < 4; ++i) {
//                     for (int d = -1; d <= 1; d += 2) {
//                         int y = ((node.charAt(i) - '0') + d + 10) % 10;
//                         String nei = node.substring(0, i) + ("" + y) + node.substring(i+1);
//                         if (!seen.contains(nei)) {
//                             seen.add(nei);
//                             queue.offer(nei);
//                         }
//                     }
//                 }

/*
class Solution {
    public int openLock(String[] deadends, String target) {
        Set<String> dead = new HashSet();
        for (String d: deadends) dead.add(d);

        Queue<String> queue = new LinkedList();
        queue.offer("0000");
        queue.offer(null);

        Set<String> seen = new HashSet();
        seen.add("0000");

        int depth = 0;
        while (!queue.isEmpty()) {
            String node = queue.poll();
            if (node == null) {
                depth++;
                if (queue.peek() != null)
                    queue.offer(null);
            } else if (node.equals(target)) {
                return depth;
            } else if (!dead.contains(node)) {
                for (int i = 0; i < 4; ++i) {
                    for (int d = -1; d <= 1; d += 2) {
                        int y = ((node.charAt(i) - '0') + d + 10) % 10;
                        String nei = node.substring(0, i) + ("" + y) + node.substring(i+1);
                        if (!seen.contains(nei)) {
                            seen.add(nei);
                            queue.offer(nei);
                        }
                    }
                }
            }
        }
        return -1;
    }
}
*/