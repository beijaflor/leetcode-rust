// https://leetcode.com/submissions/detail/546985494/
impl Solution {
    pub fn max_count(mut m: i32, mut n: i32, ops: Vec<Vec<i32>>) -> i32 {
        ops.into_iter().for_each(|v| {
            m = i32::min(m, v[0]);
            n = i32::min(n, v[1]);
        });
        
        m * n
    }
}