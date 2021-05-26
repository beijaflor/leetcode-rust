// https://leetcode.com/submissions/detail/498522700/
use std::collections::BTreeSet;

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut set: BTreeSet<char> = BTreeSet::new();
        n.chars().for_each(|c| { set.insert(c); });
        set.iter().last().unwrap().to_digit(10).unwrap() as i32
    }
}