// https://leetcode.com/submissions/detail/444898102/
impl Solution {
  pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
      nums.sort();
      let length = nums.len() - 1;
      let mut result = 0;
      let mut left_pointer = 0;
      let mut right_pointer = length;

      // println!("nums: {:?}, k: {}", nums, k);

      while left_pointer < right_pointer {
          let num = nums[left_pointer];
          let pair_value = k - num;
          while left_pointer < right_pointer {
              // println!("lp: {}, rp: {}", left_pointer, right_pointer);
              if nums[right_pointer] < pair_value {
                  break
              }
              if nums[right_pointer] == pair_value {
                  result += 1;
                  right_pointer -= 1;
                  break
              }
              right_pointer -= 1;
          }
          left_pointer += 1;
          // println!("result: {}", result);
      }
      result
  }
}
