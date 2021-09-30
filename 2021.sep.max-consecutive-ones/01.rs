// https://leetcode.com/submissions/detail/558530048/
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut current = 0;
        let mut pointer = 0;
        while pointer < nums.len() {
            if nums[pointer] == 1 {
                current += 1;
            } else {
                result = i32::max(result, current);
                current = 0;
            }
            pointer += 1;
        }
        i32::max(result, current)
    }
}