// https://leetcode.com/submissions/detail/444390426/
fn count(n: i32, last: i32) -> i32 {
  // println!("n: {}, last: {}", n, last);
  let mut ret = 0;
  if n == 0 { return 1 }
  for index in last..5 {
      ret += count(n - 1, index);
  }
  ret
}

impl Solution {
  pub fn count_vowel_strings(n: i32) -> i32 {
      count(n, 0)
  }
}
