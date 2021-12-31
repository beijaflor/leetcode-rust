// https://leetcode.com/submissions/detail/604726889/
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n == 0 { false }
        else {
            // while n % 2 == 0 {
            //     n /= 2;
            // }
            // n == 1
            
            let x = n as i64;
            (x & (x - 1)) == 0
        }
    }
}