// https://leetcode.com/submissions/detail/503316176/
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut dead: HashSet<Vec<u8>> = HashSet::new();
        deadends.into_iter().for_each(|d| {
            dead.insert(d.chars().map(|c| c as u8 - '0' as u8).collect());
        });
        
        let target: Vec<u8> = target.chars().map(|c| c as u8 - '0' as u8).collect();
        
        // println!("dead: {:?}", dead);
        
        let mut queue: VecDeque<Vec<u8>> = VecDeque::new();
        queue.push_back(vec![0,0,0,0]);
        queue.push_back(vec![]);
        
        let mut seen: HashSet<Vec<u8>> = HashSet::new();
        seen.insert(vec![0,0,0,0]);
        
        let mut depth = 0;
        
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            if node.is_empty() {
                depth += 1;
                if let Some(node) = queue.front() {
                    if !node.is_empty() { queue.push_back(vec![]) }
                }
            } else if node == target {
                return depth
            } else if !dead.contains(&node) {
                for i in 0..4 {
                    for d in (-1..2).step_by(2) {
                        let mut nei = node.clone();
                        nei[i] = (*node.iter().nth(i).unwrap() as u8 + d as u8 + 10) % 10;
                        // println!("nei: {:?}", nei);
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