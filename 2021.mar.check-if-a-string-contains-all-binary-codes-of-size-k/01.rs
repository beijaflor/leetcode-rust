//
use std::collections::BTreeSet;
// https://leetcode.com/submissions/detail/467283550
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false
        }
        let mut set: BTreeSet<&str> = BTreeSet::new();
        for index in 0..(s.len() + 1 - k as usize) {
            set.insert(&s[index..(index + k as usize)]);
        }
        println!("{}", set.len());
        set.len() >= (0..k).fold(1, |acc, _| acc * 2) as usize
    }
}