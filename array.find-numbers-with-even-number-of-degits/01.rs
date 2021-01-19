// https://leetcode.com/submissions/detail/437462895/
impl Solution {
  pub fn find_numbers(nums: Vec<i32>) -> i32 {
      let mut count: i32 = 0;
      let mut iterator = nums.into_iter();
      while let Some(num) = iterator.next() {
          if num.to_string().len() % 2 == 0 {
              count += 1;
          }
      }
      count
  }
}
