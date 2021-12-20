// https://leetcode.com/submissions/detail/596175758/
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut iterator = nums.into_iter();
        let mut result = iterator.next().unwrap();
        let mut max_so_far = result;
        let mut min_so_far = result;
        iterator.for_each(|num| {
            let current_max = max_so_far * num;
            let current_min = min_so_far * num;
            max_so_far = i32::max(num, i32::max(current_max, current_min));
            min_so_far = i32::min(num, i32::min(current_max, current_min));
            result = i32::max(max_so_far, result);
        });
        result
    }
}