// https://leetcode.com/submissions/detail/437457996/
impl Solution {
  pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
      let chars: Vec<char> = nums.into_iter().map(|num| std::char::from_digit(num as u32, 10).unwrap()).collect();
      let string: String = chars.into_iter().collect();
      let ones: Vec<&str> = string.split("0").collect();
      let max = ones.into_iter().fold(0, |acc, str| acc.max(str.len()));
      max as i32
  }
}
