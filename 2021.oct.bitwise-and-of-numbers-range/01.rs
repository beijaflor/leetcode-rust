// https://leetcode.com/submissions/detail/568692618/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right { return left }
        let mut result = left;
        ((left + 1)..=right).for_each(|num| {
            result &= num;
        });
        result
    }
}