// https://leetcode.com/submissions/detail/446175035/
use std::collections::VecDeque;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = VecDeque::with_capacity(k as usize);
        result.push_back(nums[0]);
        let mut pointer = 1;

        while pointer < nums.len() {
            if result.len() != 0 && !(nums.len() as i32 - pointer as i32 - (k - result.len() as i32) < 1) && nums[pointer] < result[result.len() - 1] {
                result.pop_back();
            } else {
                result.push_back(nums[pointer]);
                pointer += 1;
            }
        }
        // println!("result: {:?}", result);
        for _ in 0..result.len() - k as usize {
            result.pop_back();
        }
        result.into_iter().collect()
    }
}