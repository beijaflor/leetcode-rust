// https://leetcode.com/submissions/detail/441881412
impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
      if m == 0 {
          for i in 0..(n as usize) {
              nums1[i] = nums2[i];
          }
          return
      } 
      if n == 0 { return }

      let mut pointer1 = (m - 1) as usize;
      let mut pointer2 = (n - 1) as usize;

      for index in (0..nums1.len()).rev() {
          if nums1[pointer1] > nums2[pointer2] {
              nums1[index] = nums1[pointer1];
              if pointer1 == 0 {
                  for i in 0..(index) {
                      nums1[i] = nums2[i];
                  }
                  return
              } else { pointer1 -= 1 }
          } else {
              nums1[index] = nums2[pointer2];
              if pointer2 == 0 { return } else { pointer2 -= 1 }
          }
      }
  }

}
