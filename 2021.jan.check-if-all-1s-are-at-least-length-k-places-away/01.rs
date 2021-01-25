// https://leetcode.com/submissions/detail/447657536/
impl Solution {
  pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
      if nums.len() == 1 {
          return true
      }

      let ones = nums.iter().enumerate().fold(vec![], |mut acc, (index, v)| {
          if *v == 1 {
              acc.push(index);
          }
          acc
      });

      if ones.len() == 0 {
          return true
      }

      let mut last_pos = 0;
      let mut min = nums.len();
      for index in 1..ones.len() {
          let diff = ones[index] - last_pos - 1;
          if min > diff {
              min = diff;
          }
          last_pos = ones[index];
      }
      
      k <= min as i32
  }
}