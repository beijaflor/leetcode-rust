// https://leetcode.com/submissions/detail/450781902/
impl Solution {
  pub fn hammingWeight (mut n: u32) -> i32 {
      let mut result = 0;
      while n != 0 {
          if n % 2 == 1 {
            result += 1
          }
          n /= 2;
      }
      result
  }
}