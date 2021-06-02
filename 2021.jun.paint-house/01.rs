// https://leetcode.com/submissions/detail/501832485/
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        *costs.into_iter().fold([0, 0, 0], |mut acc, current| {
            let a = current[0] + i32::min(acc[1], acc[2]);
            let b = current[1] + i32::min(acc[0], acc[2]);
            let c = current[2] + i32::min(acc[0], acc[1]);
            acc[0] = a;
            acc[1] = b;
            acc[2] = c;
            acc
        }).iter().min().unwrap()
    }
}