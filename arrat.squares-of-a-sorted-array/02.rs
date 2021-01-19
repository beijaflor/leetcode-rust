// https://leetcode.com/submissions/detail/437515548/
impl Solution {
  pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
      let mut result: Vec<i32> = vec!();
      let mut neg_index = 0;
      let mut pos_index = nums.len() - 1;
      while neg_index != pos_index {
          let neg = nums[neg_index] * nums[neg_index];
          let pos = nums[pos_index] * nums[pos_index];
          if neg < pos {
              result.insert(0, pos);
              pos_index -= 1;
          } else {
              result.insert(0, neg);
              neg_index += 1;
          }
      }
      result.insert(0, nums[pos_index] * nums[pos_index]);
      result
  }
}
