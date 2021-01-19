// https://leetcode.com/submissions/detail/439403249/
impl Solution {
  pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
      let mut fallback_result = 0;
      let itarator = arr.into_iter().enumerate();
      for (index, num) in itarator {
          let diff = num - (index as i32);
          if diff > k {
              return num - ( diff - k )
          } else {
              fallback_result = num + 1 + (k - diff)
          }
      };
      fallback_result
  }
}
