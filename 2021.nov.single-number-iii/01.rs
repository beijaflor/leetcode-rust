// https://leetcode.com/submissions/detail/582718116/
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut set = HashSet::<i32>::new();
        nums.into_iter().for_each(|num| {
            if !set.remove(&num) {
                set.insert(num);
            }
        });
        set.into_iter().collect()
    }
}