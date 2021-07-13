// https://leetcode.com/submissions/detail/521824927/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut last = std::i32::MIN;
        for index in 0..nums.len() - 1 {
            if last < nums[index] && nums[index] > nums[index + 1] {
                return index as i32
            }
            last = nums[index];
        }
        return (nums.len() - 1) as i32
    }
}