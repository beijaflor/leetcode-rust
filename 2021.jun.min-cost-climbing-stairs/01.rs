// https://leetcode.com/submissions/detail/504407798/
impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        (0..cost.len()-2).rev().for_each(|index| {
            cost[index] = cost[index] + i32::min(cost[index+1], cost[index+2]);
        });
        i32::min(cost[0], cost[1])
    }
}