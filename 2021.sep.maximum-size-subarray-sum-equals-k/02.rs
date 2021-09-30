// https://leetcode.com/submissions/detail/563201363/
use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut longest = 0;
        let mut indices: HashMap<i32, usize>  = HashMap::new();
        
        (0..nums.len()).for_each(|index| {
            sum += nums[index];
            if sum == k {
                longest = index + 1;
            }
            
            if let Some(last_index) = indices.get(&(sum - k)) {
                longest = usize::max(longest, index - last_index);
            }
            
            if let Some(_) = indices.get(&sum) {
                // NOP
            } else {
                indices.insert(sum, index);
            }
        });
        
        longest as i32
    }
}