// https://leetcode.com/submissions/detail/587572205/
use std::collections::VecDeque;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let N = nums.len();
        if N == 0 {
            return vec![]
        }

        let mut dp = vec![0; N];

        nums.sort();

        let mut max_size = -1;
        let mut max_index = -1;

        (0..N).for_each(|index1| {
            let mut size = 0;
            (0..index1).for_each(|index2| {
                if nums[index1] % nums[index2] == 0 && size < dp[index2] {
                    size = dp[index2]
                }
            });

            dp[index1] = size + 1;

            if max_size < dp[index1] {
                max_size = dp[index1];
                max_index = index1 as i32;
            }
        });

        // println!("{:?} {} {}", dp, max_size, max_index);

        let mut subset = VecDeque::<i32>::new();
        let mut current_size = max_size;
        let mut current_tail = nums[max_index as usize];
        for index in (0..=max_index as usize).rev() {
            if current_size == 0 {
                break
            }

            if current_tail % nums[index] == 0 && current_size == dp[index] {
                subset.push_front(nums[index]);
                current_tail = nums[index];
                current_size -= 1;
            }
        };

        subset.into_iter().collect()
    }
}