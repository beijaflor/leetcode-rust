// https://leetcode.com/submissions/detail/437122571/
impl Solution {
  pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
      let mut max: i32 = 0;
      let mut current: i32 = 0;
      let mut iterator = nums.into_iter();
      while let Some(num) = iterator.next() {
          if num == 1 {
              current += 1;
          } else {
              if current > max {
                  max = current;
              }
              current = 0;
          }
      }
      if current > max {
          return current
      }
      max
  }
}
