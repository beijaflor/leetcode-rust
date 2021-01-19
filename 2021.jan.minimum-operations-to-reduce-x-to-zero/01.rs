// https://leetcode.com/submissions/detail/443234486
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total = nums.iter().fold(0, |acc, num| acc + num);
        let max = nums.len() as i32;
        let mut result: i32 = -1;
        let mut left_pointer = 0;
        let mut current = 0;
        println!("total: {}", total);
        
        for right_pointer in 0..(max) {
            current += nums[right_pointer as usize];
            while current > total - x && left_pointer <= right_pointer {
                current -= nums[left_pointer as usize];
                left_pointer += 1;
            }
            if current == total - x {
                let step: i32 = right_pointer - left_pointer + 1;
                result = if result < step { step } else { result };
            }
        }
        return if result != -1 {
            max - result
        } else {
            -1
        } 
    }
}
