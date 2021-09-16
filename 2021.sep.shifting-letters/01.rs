// https://leetcode.com/submissions/detail/551511955/
use std::collections::VecDeque;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut offset = 0;
        let mut sum_shifts: VecDeque<i32> = VecDeque::new();
        shifts.into_iter().rev().for_each(|s| {
            offset = (offset + s as i32) % 26;
            sum_shifts.push_front(offset);
        });
        
        // println!("{:?}", sum_shifts);
        
        s.as_bytes().to_vec().into_iter().map(|b| {
            // println!("{}", b);
            let shift = sum_shifts.pop_front().unwrap();
            ((b - 97 + shift as u8) % 26 + 97) as char
        }).collect()
    }
}