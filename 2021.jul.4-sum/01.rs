// https://leetcode.com/submissions/detail/523459108/
use std::collections::HashSet;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 { return vec![] }
        nums.sort();
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        (0..nums.len() - 3).for_each(|p1| {
            (p1 + 1..nums.len() - 2).for_each(|p2| {
                (p2 + 1..nums.len() - 1).for_each(|p3| {            
                    (p3 + 1..nums.len()).for_each(|p4| {
                        if nums[p1] + nums[p2] + nums[p3] + nums[p4] == target {
                            result.insert(vec![nums[p1], nums[p2], nums[p3], nums[p4]]);
                        }
                    });
                });
            });
        });
        
        result.into_iter().collect::<Vec<Vec<i32>>>()
    }
}