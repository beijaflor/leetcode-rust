// https://leetcode.com/submissions/detail/486675087/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = nums.iter().position(|num| *num == target);
        if let Some(start) = start {
            let end = nums.len() - nums.iter().rev().position(|num| *num == target).unwrap() - 1;
            vec![start as i32, end as i32]
        } else {
            vec![-1, -1]
        }
    }
}