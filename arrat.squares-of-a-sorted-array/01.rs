// https://leetcode.com/submissions/detail/437466739/
impl Solution {
  pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
      let mut mapped: Vec<i32> = nums.into_iter().map(|num| num * num).collect();
      mapped.sort();
      mapped
  }
}
