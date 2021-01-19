// https://leetcode.com/submissions/detail/437463747/
fn count_degits(mut num: i32) -> i32 {
  let mut digits = 1;
  while num >= 10 {
      num /= 10;
      digits += 1;
  } 
  digits
}

impl Solution {
  pub fn find_numbers(nums: Vec<i32>) -> i32 {
      let mut count: i32 = 0;
      let mut iterator = nums.into_iter();
      while let Some(num) = iterator.next() {
          if count_degits(num) % 2 == 0 {
              count += 1;
          }
      }
      count
  }
}
