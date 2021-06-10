// https://leetcode.com/submissions/detail/505593095/
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_result(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = nums.len();
        let mut score = nums[0];
        let mut pq: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        pq.push((nums[0], 0));

        (1..n).for_each(|index| {
            // println!("{:?}, {}", pq, index - k);
            while (pq.peek().unwrap().1 as i32) < (index as i32 - k as i32) {
                pq.pop();
            }
            score = nums[index] + pq.peek().unwrap().0;
            pq.push((score, index))
        });
        
        score
    }
}
