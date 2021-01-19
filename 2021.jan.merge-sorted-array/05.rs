// https://leetcode.com/submissions/detail/441889089/
impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
      let mut pointer1 = m as usize;
      let mut pointer2 = n as usize;

      for index in (0..nums1.len()).rev() {
          if pointer1 == 0 {
              for i in 0..(index + 1) {
                  nums1[i] = nums2[i];
              }
              return
          }
          if pointer2 == 0 { return }

          if nums1[pointer1 - 1] < nums2[pointer2 - 1] {
              nums1[index] = nums2[pointer2 - 1];
              pointer2 -= 1;
          } else {
              nums1[index] = nums1[pointer1 - 1];
              pointer1 -= 1;
          }
      }
  }
}
