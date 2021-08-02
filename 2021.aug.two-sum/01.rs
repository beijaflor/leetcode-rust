// https://leetcode.com/submissions/detail/532101247/
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        nums.iter().enumerate().for_each(|(index, num)| {
            map.insert(*num, index);
        });
        for index in 0..nums.len() {
            let diff = target - nums[index];
            if let Some(index2) = map.get(&diff) {
                if *index2 != index {
                    return vec![index as i32, *index2 as i32]
                }
            }
        }
        panic!("shouldn't reach this");
    }
}