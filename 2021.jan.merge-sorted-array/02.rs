// https://leetcode.com/submissions/detail/441869459/
const MIN: i32 = -1_000_000_000;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut pointer1 = m as usize;
        let mut pointer2 = n as usize;

        for index in (0..nums1.len()).rev() {
            let v1 = if pointer1 != 0 { nums1[pointer1 - 1] } else { MIN };
            let v2 = if pointer2 != 0 { nums2[pointer2 - 1] } else { MIN };
            println!("left: {:?} at {:?}, right: {:?} at {:?}", v1, pointer1, v2, pointer2);
            if v1 > v2 {
                nums1[index] = v1;
                pointer1 -= 1;
            } else {
                nums1[index] = v2;
                if pointer2 != 0 {
                    pointer2 -= 1;
                } else {
                    pointer1 -= 1;
                }
            }
        }
    }
}
