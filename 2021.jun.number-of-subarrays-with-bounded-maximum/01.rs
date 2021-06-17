// https://leetcode.com/submissions/detail/509207107
fn count(nums: &Vec<i32>, bound: i32) -> i32 {
    let mut result = 0;
    let mut current = 0;
    for num  in nums.iter() {
        current = if num <= &bound { current + 1 } else { 0 };
        result += current;
    }
    result
}

impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        count(&nums, right) - count(&nums, left - 1)
    }
}