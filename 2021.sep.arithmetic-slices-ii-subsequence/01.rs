// https://leetcode.com/submissions/detail/552655236/
use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result: i64 = 0;
        let mut count: Vec<HashMap<i32, i32>> = Vec::with_capacity(n);
        for index in (0..n) {
            count.push(HashMap::new());
            for j in 0..index {
                let delta = nums[index] as i64 - nums[j] as i64;
                // println!("{}, {}", delta, i32::MIN as i64);
                if delta < i32::MIN as i64 || delta > i32::MAX as i64 {
                    continue
                }
                let diff = delta as i32;
                let sum = *count[j].get(&diff).unwrap_or(&0);
                let origin = *count[index].get(&diff).unwrap_or(&0);
                count[index].insert(diff, origin + sum + 1);
                result += sum as i64;
            };
        };
        
        result as i32
    }
}