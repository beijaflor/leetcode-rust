// https://leetcode.com/submissions/detail/488949319
impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut violation = 0;
        for index in 1..nums.len() {
            if nums[index - 1] > nums[index] {
                if violation == 1 {
                    return false
                }
                
                violation += 1;
                
                if index < 2 || nums[index -2] <= nums[index] {
                    nums[index - 1] = nums[index];
                } else {
                    nums[index] = nums[index - 1];
                }
            }
        }
        true
    }
}