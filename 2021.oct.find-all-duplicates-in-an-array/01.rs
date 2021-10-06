// https://leetcode.com/submissions/detail/566500591/
use std::collections::HashSet;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut set: HashSet<i32> = HashSet::new();
        nums.into_iter().for_each(|num| {
            if !set.insert(num) {
                result.push(num);
            }
        });
        result
    }
}