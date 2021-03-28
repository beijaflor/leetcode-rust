// https://leetcode.com/submissions/detail/473310586/
use std::collections::VecDeque;

#[derive(Debug)]
struct Palin {
    start_at: usize,
    length: usize,
    all_same: bool,
}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut result = 0;
        let mut queue: VecDeque<Palin> = VecDeque::new();
        let chars = s.chars().collect::<Vec<char>>();
        chars.iter().enumerate().for_each(|(index, _)| queue.push_back({ Palin { start_at: index, length: 1, all_same: true }}));
        while let Some(p) = queue.pop_front() {
            // println!("{:?}", p);
            result += 1;
            if p.start_at + p.length == chars.len() { continue }
            if p.all_same && chars[p.start_at] == chars[p.start_at + p.length] {
                queue.push_back({ Palin { start_at: p.start_at, length: p.length + 1, all_same: true }});
                continue
            }
            if p.start_at == 0 { continue }
            if chars[p.start_at - 1] == chars[p.start_at + p.length] {
                queue.push_back({ Palin { start_at: p.start_at - 1, length: p.length + 2, all_same: false }});
            }
        }
        result
    }
}