// https://leetcode.com/submissions/detail/565920477/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 { return n }
        (3..=n).fold((1, 2), |(first, second), num| {
            (second, first + second)
        }).1
    }
}