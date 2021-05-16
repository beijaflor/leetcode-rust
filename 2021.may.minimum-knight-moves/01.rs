// https://leetcode.com/submissions/detail/493703888/
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        println!("x: {}, y: {}", x, y);
        let x = x.abs();
        let y = y.abs();
        if x + y == 0 {
            return 0
        }
        if x + y == 1 {
            return 3
        }
        if x + y == 2 {
            return 2
        }
        if (x + y) / 3 < 3 {
            // println!("is close to");
            return 1 + i32::min(Solution::min_knight_moves(x - 2, y - 1), Solution::min_knight_moves(x - 1, y - 2));
        }
        if x > y {
            1 + Solution::min_knight_moves(x - 2, y - 1)
        } else {
            1 + Solution::min_knight_moves(x - 1, y - 2)
        }
    }
}