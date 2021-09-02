// https://leetcode.com/submissions/detail/547133595/
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        nums.into_iter().min().unwrap()
    }
}