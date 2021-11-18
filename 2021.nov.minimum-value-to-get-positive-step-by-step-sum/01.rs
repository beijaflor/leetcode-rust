// https://leetcode.com/submissions/detail/585298511/
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        nums.into_iter().fold(0, |acc, step| {
            let next = acc + step;
            min = i32::min(min, next);
            next
        });
        if min < 1 {
            min.abs() + 1
        } else {
            1
        }
    }
}