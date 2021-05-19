// https://leetcode.com/submissions/detail/495282477/
impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let median = nums[nums.len() / 2];
        nums.into_iter().fold(0, |acc, num| {
            acc + (median - num).abs()
        })
    }
}