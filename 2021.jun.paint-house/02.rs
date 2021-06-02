// https://leetcode.com/submissions/detail/501833313/
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        *costs.into_iter().fold([0, 0, 0], |mut acc, current| {
            let clone = acc.clone();
            acc[0] = current[0] + i32::min(clone[1], clone[2]);
            acc[1] = current[1] + i32::min(clone[0], clone[2]);
            acc[2] = current[2] + i32::min(clone[0], clone[1]);
            acc
        }).iter().min().unwrap()
    }
}