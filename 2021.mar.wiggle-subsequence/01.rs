// https://leetcode.com/submissions/detail/469656011/
use std::cmp::Ordering::{ Less, Greater, Equal};

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut up: Vec<i32> = Vec::with_capacity(nums.len());
        let mut down: Vec<i32> = Vec::with_capacity(nums.len());
        up.push(1);
        down.push(1);
        (1..nums.len()).for_each(|index| {
            match nums[index].cmp(&nums[index - 1]) {
                Equal => {
                    up.push(up[index -1]);
                    down.push(down[index -1]);
                },
                Greater => {
                    up.push(down[index -1] + 1);
                    down.push(down[index -1]);
                },
                Less => {
                    up.push(up[index -1]);
                    down.push(up[index -1] + 1);
                },
            }
        });
        i32::max(*up.last().unwrap(), *down.last().unwrap())
    }
}