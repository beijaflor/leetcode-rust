// https://leetcode.com/submissions/detail/488530530/
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        // let mut result = Vec::with_capacity(nums.len());
        // result.push(nums[0]);
        for index in 1..nums.len() {
            // result.push(nums[index] + result[index - 1]);
            nums[index] = nums[index] + nums[index - 1];
        }
        nums
    }
}