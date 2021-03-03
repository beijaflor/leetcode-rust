// https://leetcode.com/submissions/detail/463053740/
impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for index in 0..nums.len() {
            if nums[index] != index as i32 {
                return index as i32
            }
        }
        nums.len() as i32
    }
}