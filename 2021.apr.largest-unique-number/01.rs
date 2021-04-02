// https://leetcode.com/submissions/detail/475072340/
use std::collections::BTreeSet;

impl Solution {
    pub fn largest_unique_number(a: Vec<i32>) -> i32 {
        let mut duplicate_set: BTreeSet<i32> = BTreeSet::new();
        let mut candidate_set: BTreeSet<i32> = BTreeSet::new();
        a.into_iter().for_each(|v| {
            let is_not_duplicate = duplicate_set.insert(v);
            if is_not_duplicate {
                candidate_set.insert(v);
            } else {
                candidate_set.remove(&v);
            }
        });
        println!("{:?}", candidate_set);
        match candidate_set.iter().last() {
            None => -1,
            Some(candidate) => *candidate
        }
    }
}