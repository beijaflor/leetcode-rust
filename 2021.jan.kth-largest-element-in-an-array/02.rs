// https://leetcode.com/submissions/detail/443941100/
use std::collections::BinaryHeap;

impl Solution {
    // use BinaryHeap as a min heap
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut q = BinaryHeap::new();
        for num in nums {
            q.push(-num);
            if q.len() as i32 > k {
                q.pop();
            }
        }
        -(q.into_vec().first().unwrap())
    }
}
