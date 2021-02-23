// https://leetcode.com/submissions/detail/458884427/
impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        if x > y {
            return x - y
        }
        let mut result = 0;
        let mut goal = y;
        while goal != x {
            // println!("goal: {}, result: {}", goal, result);
            result += 1;
            if goal < x {
                // println!("{}", result + x - goal - 1);
                return result + x - goal - 1
            } else if goal % 2 == 1 {
                goal += 1;
            } else {
                goal /= 2;
            }
        }
        result
    }
}