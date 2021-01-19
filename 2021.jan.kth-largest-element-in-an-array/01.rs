// https://leetcode.com/submissions/detail/443928504/
impl Solution {
  pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
      let mut tmp = nums.clone();
      // tmp.dedup();
      tmp.sort();
      let unq: Vec<i32> = tmp.into_iter().rev().collect();
      let target_num = unq[(k - 1) as usize];
      // println!("unique: {:?}, k: {}, target_num: {}", unq, k, target_num);

      target_num
  }
}
