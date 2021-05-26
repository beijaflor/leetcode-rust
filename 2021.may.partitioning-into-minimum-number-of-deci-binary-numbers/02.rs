// https://leetcode.com/submissions/detail/498521704/
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut s = n.chars().collect::<Vec<char>>();
        s.sort();
        s.pop().unwrap().to_digit(10).unwrap() as i32
    }
}