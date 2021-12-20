// https://leetcode.com/submissions/detail/597587641/
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let (odd, even) = position.into_iter().fold((0, 0), |(odd, even), pos| {
            if pos % 2 == 0 {
                (odd, even + 1)
            } else {
                (odd + 1, even)
            }
        });
        
        i32::min(odd, even)
    }
}