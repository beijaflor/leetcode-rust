// https://leetcode.com/submissions/detail/446169938/
use std::collections::VecDeque;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = VecDeque::with_capacity(k as usize);
        result.push_back(nums[0]);
        let mut pointer = 1;

        while pointer < nums.len() {
            if nums.len() - pointer - ( k as usize - result.len() ) < 1 {
                result.push_back(nums[pointer]);
            } else if result.len() != 0 && nums[pointer] < result[result.len() - 1] {
                result.pop_back();
                pointer -= 1;
            } else if result.len() < k as usize {
                result.push_back(nums[pointer]);
            }
            pointer += 1;
        }
        // println!("result: {:?}", result);
        result.into_iter().collect()
    }
}