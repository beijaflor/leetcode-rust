// https://leetcode.com/submissions/detail/462388448/
use std::collections::BTreeSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut uniq = BTreeSet::new();
        candy_type.iter().for_each(|v| {
            uniq.insert(*v);
        });
        // println!("{:?}, {:?}", candy_type, uniq);
        usize::min(candy_type.len() / 2, uniq.len()) as i32
    }
}