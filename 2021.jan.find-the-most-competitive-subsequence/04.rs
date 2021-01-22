// https://leetcode.com/submissions/detail/446176366/
pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
  let mut result = Vec::with_capacity(k as usize);
  let mut pointer = 0;

  while pointer < nums.len() {
      if result.len() != 0 && !(nums.len() as i32 - pointer as i32 - (k - result.len() as i32) < 1) && nums[pointer] < result[result.len() - 1] {
          result.remove(result.len() - 1);
      } else {
          result.push(nums[pointer]);
          pointer += 1;
      }
  }
  result[0..(k as usize)].to_vec()
}