// https://leetcode.com/submissions/detail/609565082/
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k % 2 == 0 { return -1 }
        let mut remainder = 0;
        let mut length = 1;
        while length <= k {
            remainder = (remainder * 10 + 1) % k;
            if remainder == 0 {
                return length
            }
            length += 1;
        }
        -1
    }
}