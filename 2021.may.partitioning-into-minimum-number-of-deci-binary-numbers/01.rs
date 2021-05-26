// https://leetcode.com/submissions/detail/498520752/
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut s = n.chars().collect::<Vec<char>>();
        s.sort();
        s.pop().unwrap().to_string().parse::<i32>().unwrap()
    }
}