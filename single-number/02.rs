impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
      let mut result: Vec<i32> = vec![];
      let mut iterator = nums.into_iter();
      while let Some(num) = iterator.next() {
          if let Some(index) = result.iter().position(|&i| i == num ) {
              result.remove(index);
          } else {
              result.push(num)
          }
      }
      result[0]
  }
}
