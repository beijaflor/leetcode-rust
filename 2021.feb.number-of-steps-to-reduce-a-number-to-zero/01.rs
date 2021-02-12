// https://leetcode.com/submissions/detail/455192770/
impl Solution {
    pub fn number_of_steps (mut num: i32) -> i32 {
        let mut count = 0;
        while num != 0 {
            count += 1;
            if num % 2 == 1 {
                num -= 1;
            } else {
                num /= 2;
            }
        }
        count
    }
}