//https://leetcode.com/submissions/detail/448574816/
fn mod_binary(n: i32) -> i32 {
  let mut x = n;
  let mut result = 1;
  while x > 0 {
      x /= 2;
      result *= 2;
  }
  result
}

impl Solution {
  pub fn concatenated_binary(n: i32) -> i32 {
      let mut result: i128 = 0;
      for index in 1..(n + 1) {
          result = result * mod_binary(index) as i128 + index as i128;
          result %= 1_000_000_007;
      }
      result as i32
  }
}