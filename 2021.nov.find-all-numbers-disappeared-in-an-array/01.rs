// https://leetcode.com/submissions/detail/588907488/
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut len = nums.len() as i32;
        let map = nums.into_iter().collect::<std::collections::HashSet<i32>>();
        (1..=len).filter(|index| !map.get(index).is_some()).collect()
    }
}