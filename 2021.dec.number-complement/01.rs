// https://leetcode.com/submissions/detail/607841985/
impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut mask = num;
        mask |= mask >> 1;
        mask |= mask >> 2;
        mask |= mask >> 4;
        mask |= mask >> 8;
        mask |= mask >> 16;
        mask ^ num
    }
}