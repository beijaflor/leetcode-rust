impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
      let mut sum_num: i32 = 0;
      let mut sum_set: i32 = 0;
      let mut set: Vec<i32> = vec![];
      let mut iterator = nums.into_iter();
      while let Some(num) = iterator.next() {
          if !set.contains(&num) {
              set.push(num);
              sum_set += num;
          }
          sum_num += num;
      }
      2 * sum_set - sum_num
  }
}
