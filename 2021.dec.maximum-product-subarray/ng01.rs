// https://leetcode.com/submissions/detail/596172046/
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0] }
        let mut max = *nums.iter().max().unwrap_or(&i32::MIN);
        (0..nums.len()).for_each(|start| {
            ((start + 1)..nums.len()).for_each(|end| {
                let current = nums[start..=end].iter().fold(1, |acc, curr| {
                    acc * curr
                });
                // println!("{}", current);
                max = i32::max(max, current);
            });
        });
        max
    }
}