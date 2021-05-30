// https://leetcode.com/submissions/detail/500353630/
impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return 0 }
        nums.sort();
        let mut iterator = nums.into_iter();
        let start = iterator.next().unwrap();
        iterator.fold((start, 0), |(last, max), current| (current, i32::max(max, (last - current).abs()))).1
    }
}